// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::{fs, fmt::Display, cmp::Ordering};
use crate::percentage;
use super::common::{Result, draw_line};

/// Convert string to decimal integer.
macro_rules! atoi {
    ($int_type: ty, $str_value: expr, $radix: expr) => {
        <$int_type>::from_str_radix($str_value, $radix)?
    };
    ($int_type: ty, $str_value: expr) => {
        <$int_type>::from_str_radix($str_value, 10)?
    };
}

/// Information loaded from /proc/pid/cmdline.
pub struct ProcCmd {
    /// Program name.
    name: String,
    /// Program arguments.
    args: Vec<String>
}

impl ProcCmd {
    fn parse_arg(s: &str) -> String {
        let tokens = s.split_once(" ");
        if tokens.is_none() {
            "-".to_string() + &s.to_string()
        } else {
            "-".to_string() + &tokens.as_ref().unwrap().0.to_string() 
                            + " \'" 
                            + &tokens.as_ref().unwrap().1.trim_end().to_string()
                            + "\'"
        }

    }
}

impl Display for ProcCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n{} {}",
            draw_line('-', "Command Line", 30).unwrap(),
            self.name,
            self.args.join(" ")
        )
    }
}

/// Information taken from /proc/pid/status.
#[derive(Default, Debug)]
pub struct ProcStatus {
    /// PID
    pid: u32,
    /// Peak virtual memory size in KB. (may < vm_size)
    vm_peak: u64,
    /// Total virtual memory size in KB.
    vm_size: u64,
    /// Peak resident set size in KB. (may < vm_rss)
    vm_hwm: u64,
    /// Resident set size in KB. (rss_anon + rss_file + rss_shmem)
    vm_rss: u64,
    /// Size of resident anonymous memory in KB.
    rss_anon: u64,
    /// Size of resident file mappings in KB.
    rss_file: u64,
    /// Size of resident shmem memory in KB.
    rss_shmem: u64,
    /// Size of private data segments in KB.
    vm_data: u64,
    /// Size of stack segments in KB.
    vm_stk: u32,
    /// Size of text segment in KB.
    vm_exe: u32,
    /// Size of shared library code in KB.
    vm_lib: u32,
    /// Size of page table entries in KB.
    vm_pte: u32,
    /// Size of hugetlb memory portions.
    hugetlb_pages: u64,
    /// Number of threads.
    threads: u32,
}

impl Display for ProcStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}
Pid:            {}
VmRSS:          {} KB
 -> RssAnon:    {} KB   ({}%)
 -> RssFile:    {} KB
 -> RssShmem:   {} KB
HugetlbPages:   {} KB",
            draw_line('-', "Process Status", 30).unwrap(),
            self.pid,
            self.vm_rss,
            self.rss_anon,
            percentage!(self.rss_anon, self.vm_rss),
            self.rss_file,
            self.rss_shmem,
            self.hugetlb_pages
        )
    }
}


/// A virtual memory area section in /proc/pid/smaps.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SmapsVma {
    /// Vma start virtual address.
    start: u64,
    /// Vma end virtual address.
    end: u64,
    /// Vma permissions.
    perm: String,
    /// Offset in the mapped file, 0 or start/PAGE_SIZE for anonymous mapping.
    offset: u64,
    /// Device number, 00:00 for anonymous mapping.
    device: String,
    /// Inode number, 0 for anonymous mapping.
    inode: u64,
    /// File path or segment description.
    notion: Option<String>,

    /// Resident set size for this VMA in KB.
    /// (Shared_Clean + Shared_Dirty + Private_Clean + Private_Dirty)
    rss: u64,
    /// Proportional set size in KB.
    /// (Private_Clean + Private_Dirty + 1/n Shared_Clean + 1/n Shared_Dirty)
    /// 
    /// As shared libraries are shared by multiple processes,
    /// pss will show the propotional memory usage. 
    pss: u64,
    /// Size of shared clean memory in KB.
    shared_clean: u64,
    /// Size of shared dirty memory in KB. (Should be written back to the swap partition)
    shared_dirty: u64,
    /// Size of private clean memory in KB.
    private_clean: u64,
    /// Size of private dirty memory in KB.
    private_dirty: u64,
    /// Size of resident physical memory in KB. (Can not be swapped out)
    locked: u64
}

impl Display for SmapsVma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:X} -> {:X} RSS: {:>10} KB {}",
            self.start,
            self.end,
            self.rss,
            match &self.notion { Some(s) => &s, None => "" },
        )
    }
}

impl Ord for SmapsVma {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rss != other.rss {
            other.rss.cmp(&self.rss)
        } else {
            if self.notion.is_some() {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

impl PartialOrd for SmapsVma {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SmapsVma {
    /// Check whether a new vma is encountered.
    fn is_another_vma(line: &str) -> bool {
        line.len() > 45 && (&line[0..12]).chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Convert strings into SmapsVma structure.
    fn from(lines: &Vec<String>) -> Result<Self> {
        let mut vma: SmapsVma = Default::default();
        // Parse the first line.
        let tokens: Vec<&str> = lines[0]
            .split_terminator(&[' ', '\t'])
            .filter(|s| s.len() > 0)
            .collect();
        let range: Vec<&str> = tokens[0].split("-").collect();
        vma.start = atoi!(u64, range[0], 16);
        vma.end = atoi!(u64, range[1], 16);
        vma.perm = tokens[1].to_string();
        vma.offset = atoi!(u64, tokens[2], 16);
        vma.device = tokens[3].to_string();
        vma.inode = atoi!(u64, tokens[4]);
        if tokens.len() > 5 {
            vma.notion = Some(tokens[5].to_string());
        }
        // Parse the other lines
        for line in lines[1..].iter() {
            let tokens: Vec<&str> = line
                .split_terminator(&[':', ' ', '\t'])
                .filter(|s| s.len() > 0)
                .collect();
            match tokens[0] {
                "Rss" => vma.rss = atoi!(u64, tokens[1]),
                "Pss" => vma.pss = atoi!(u64, tokens[1]),
                "Shared_Clean" => vma.shared_clean = atoi!(u64, tokens[1]),
                "Shared_Dirty" => vma.shared_dirty = atoi!(u64, tokens[1]),
                "Private_Clean" => vma.private_clean = atoi!(u64, tokens[1]),
                "Private_Dirty" => vma.private_dirty = atoi!(u64, tokens[1]),
                "Locked" => vma.locked = atoi!(u64, tokens[1]),
                _ => ()
            };
        }
        Ok(vma)
    }
}

/// Information taken from /proc/pid/smaps.
#[derive(Debug, Default)]
pub struct ProcSmaps {
    /// Vmas to be noticed.
    vmas: Vec<SmapsVma>,
    /// Resident set size for shared libs.
    libs_rss: u64,
    /// Names for shared libs.
    libs: Vec<String>,
}

impl Display for ProcSmaps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}
RSS (Shared Libs): {} KB
{} VMAs with RSS usage > 0:
", 
            draw_line('-', "Process VMAs", 30).unwrap(),
            self.libs_rss,
            self.vmas.len()         
        )?;
        for vma in &self.vmas {
            writeln!(f, "{}", vma)?;
        }
        Ok(())
    }
}

impl ProcSmaps {
    /// Add a new SmapsVma, vma with rss == 0 will be ignored.
    fn add(&mut self, vma: SmapsVma) {
        if vma.rss == 0 {
            return;
        }
        if vma.notion.is_some() && vma.notion.as_ref().unwrap().starts_with("/usr/lib64") {
            self.libs_rss += vma.rss;
            self.libs.push(vma.notion.unwrap().clone());
        } else {
            self.vmas.push(vma)
        }
    }
}

/// Load ProcCmd from /proc/pid/cmdline.
pub fn proc_pid_cmdline(pid: u32) -> Result<ProcCmd> {
    let cmd_str = fs::read_to_string(format!("/proc/{}/cmdline", pid))?.replace("\0", " ");
    let tokens: Vec<&str> = cmd_str.split(" -").collect();
    Ok(ProcCmd{
        name: tokens[0].to_string(),
        args: tokens[1..].into_iter()
                         .map(|s| ProcCmd::parse_arg(s))
                         .collect()
    })
}

/// Load /proc/pid/status into in-memory structure.
pub fn proc_pid_status(pid: u32) -> Result<ProcStatus> {
    let mut status: ProcStatus = Default::default();
    for line in fs::read_to_string(format!("/proc/{}/status", pid))?.lines() {
        let tokens: Vec<&str> = line
            .split_terminator(&[':', ' ', '\t'])
            .filter(|s| s.len() > 0)
            .collect();
        match tokens[0] {
            "Pid" => {
                status.pid = atoi!(u32, tokens[1]);
            }
            "VmPeak" => {
                status.vm_peak = atoi!(u64, tokens[1]);
            }
            "VmSize" => {
                status.vm_size = atoi!(u64, tokens[1]);
            }
            "VmHWM" => {
                status.vm_hwm = atoi!(u64, tokens[1]);
            }
            "VmRSS" => {
                status.vm_rss = atoi!(u64, tokens[1]);
            }
            "RssAnon" => {
                status.rss_anon = atoi!(u64, tokens[1]);
            }
            "RssFile" => {
                status.rss_file = atoi!(u64, tokens[1]);
            }
            "RssShmem" => {
                status.rss_shmem = atoi!(u64, tokens[1]);
            }
            "VmData" => {
                status.vm_data = atoi!(u64, tokens[1]);
            }
            "VmStk" => {
                status.vm_stk = atoi!(u32, tokens[1]);
            }
            "VmExe" => {
                status.vm_exe = atoi!(u32, tokens[1]);
            }
            "VmLib" => {
                status.vm_lib = atoi!(u32, tokens[1]);
            }
            "VmPTE" => {
                status.vm_pte = atoi!(u32, tokens[1]);
            }
            "HugetlbPages" => {
                status.hugetlb_pages = atoi!(u64, tokens[1]);
            }
            "Threads" => {
                status.threads = atoi!(u32, tokens[1]);
            }
            _ => (),
        }
    }
    Ok(status)
}

/// Load /proc/pid/smaps into in-memory structures.
pub fn proc_pid_smaps(pid: u32) -> Result<ProcSmaps> {
    let mut smaps: ProcSmaps = Default::default();
    let mut lines = Vec::new();
    for line in fs::read_to_string(format!("/proc/{}/smaps", pid))?.lines() {
        if SmapsVma::is_another_vma(line) {
            if lines.len() > 0 {
                smaps.add(SmapsVma::from(&lines)?);
                lines.clear();
            }
        }
        lines.push(line.to_string());
    }
    if lines.len() > 0 {
        smaps.add(SmapsVma::from(&lines)?);
        lines.clear();
    }
    smaps.vmas.sort_unstable();
    Ok(smaps)
}