// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::{fs, fmt::Display};
use crate::percentage;

use super::common::{Result, Inspector};

/// Information taken from /proc/pid/status
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

macro_rules! str_to_dec {
    ($int_type: ty, $str_value: expr) => {
        <$int_type>::from_str_radix($str_value, 10)?
    };
}

pub fn proc_pid_status(pid: u32) -> Result<ProcStatus> {
    let mut status: ProcStatus = Default::default();
    for line in fs::read_to_string(format!("/proc/{}/status", pid))?.lines() {
        let tokens: Vec<&str> = line.split_terminator(&[':', ' ', '\t'])
                                    .filter(|s| s.len() > 0)
                                    .collect();
        match tokens[0] {
            "Pid" => { status.pid = str_to_dec!(u32, tokens[1]); },
            "VmPeak" => { status.vm_peak = str_to_dec!(u64, tokens[1]); },
            "VmSize" => { status.vm_size = str_to_dec!(u64, tokens[1]); },
            "VmHWM" => { status.vm_hwm = str_to_dec!(u64, tokens[1]); },
            "VmRSS" => { status.vm_rss = str_to_dec!(u64, tokens[1]); },
            "RssAnon" => { status.rss_anon = str_to_dec!(u64, tokens[1]); },
            "RssFile" => { status.rss_file = str_to_dec!(u64, tokens[1]); },
            "RssShmem" => { status.rss_shmem = str_to_dec!(u64, tokens[1]); },
            "VmData" => { status.vm_data = str_to_dec!(u64, tokens[1]); },
            "VmStk" => { status.vm_stk = str_to_dec!(u32, tokens[1]); },
            "VmExe" => { status.vm_exe = str_to_dec!(u32, tokens[1]); },
            "VmLib" => { status.vm_lib = str_to_dec!(u32, tokens[1]); },
            "VmPTE" => { status.vm_pte = str_to_dec!(u32, tokens[1]); },
            "HugetlbPages" => { status.hugetlb_pages = str_to_dec!(u64, tokens[1]); },
            "Threads" => {status.threads = str_to_dec!(u32, tokens[1]); },
            _ => ()
        }
    }
    Ok(status)
}

/// Virtual memory permissions.
enum VmaPerm {
    Read,
    Write,
    Execute,
    Shared,
    Private
}

/// A virtual memory area section in /proc/pid/smaps 
struct SmapsVma {
    /// Vma start virtual address
    start: u64,
    /// Vma end virtual address
    end: u64,
    /// Vma permissions
    perm: [VmaPerm; 4],
    /// Offset in the mapped file, 0 or start/PAGE_SIZE for anonymous mapping
    offset: u64,
    /// Device number, [0, 0] for anonymous mapping
    device: [u32; 2],
    /// Inode number, 0 for anonymous mapping
    inode: u64,
    /// File path or segment description.
    notion: Option<String>
}

/// Information taken from /proc/pid/smaps
struct ProcSmaps {
    vmas: Vec<SmapsVma>
}

/// Memory information for a VM
struct MemInfo {
    status: ProcStatus
}

impl Display for MemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = &self.status;
        write!(
            f, 
"----------Status----------
Pid:            {}
VmRSS:          {} KB
 -> RssAnon:    {} KB   ({}%)
 -> RssFile:    {} KB
 -> RssShmem:   {} KB
HugetlbPages:   {} KB",
            status.pid,
            status.vm_rss,
            status.rss_anon, percentage!(status.rss_anon, status.vm_rss),
            status.rss_file,
            status.rss_shmem,
            status.hugetlb_pages
        )
    }
}

impl MemInfo {
    fn fill(pid: u32) -> Option<Self> {
        match proc_pid_status(pid) {
            Ok(status) => {
                Some(MemInfo { status })
            },
            Err(e) => {
                eprintln!("{}", e);
                None
            },
        }
    }
}

/// Inspector for VM's memory consumption.  
pub struct MemInspector {
    /// Memory information for a VM
    info: Option<MemInfo>
}

impl Inspector for MemInspector {
    fn inspect(&self) {
        if self.info.is_none() {
            return;
        }
        println!("{}", self.info.as_ref().unwrap());
    }
}

impl MemInspector {
    pub fn new(pid: u32) -> Self {
        MemInspector { info: MemInfo::fill(pid) }
    }
}