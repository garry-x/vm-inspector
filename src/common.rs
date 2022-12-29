// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::{fs, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;

/// Error generated when inspecting the VM.   
pub enum Error {
    ParseError(String),
    IoError(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            IoError(s) => write!(f, "I/O Error, error={}", s),
            ParseError(s) => write!(f, "Failed to parse process information, error={}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseError(e.to_string())
    }
}


/// Information taken from /proc/pid/status
#[derive(Default, Debug)]
pub struct ProcStatus {
    /// PID
    pub pid: u32,
    /// Peak virtual memory size in KB. (may < vm_size)
    pub vm_peak: u64,
    /// Total virtual memory size in KB. 
    pub vm_size: u64,
    /// Peak resident set size in KB. (may < vm_rss)
    pub vm_hwm: u64,
    /// Resident set size in KB. (rss_anon + rss_file + rss_shmem)
    pub vm_rss: u64,
    /// Size of resident anonymous memory in KB.  
    pub rss_anon: u64,
    /// Size of resident file mappings in KB.
    pub rss_file: u64,
    /// Size of resident shmem memory in KB. 
    pub rss_shmem: u64,
    /// Size of private data segments in KB.  
    pub vm_data: u64,
    /// Size of stack segments in KB.
    pub vm_stk: u32,
    /// Size of text segment in KB.
    pub vm_exe: u32,
    /// Size of shared library code in KB.
    pub vm_lib: u32,
    /// Size of page table entries in KB. 
    pub vm_pte: u32,
    /// Size of hugetlb memory portions. 
    pub hugetlb_pages: u64,
    /// Number of threads.  
    pub threads: u32,
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


pub trait Inspector {
    fn inspect(&self);
}
