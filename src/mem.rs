// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Display;
use crate::common::proc_pid_status;
use super::common::{ProcStatus, Inspector};

/// Memory information for a VM
struct MemInfo {
    status: ProcStatus
}

impl Display for MemInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
"----------Status----------
Pid:            {}
VmRSS:          {} KB
 -> RssAnon:    {} KB
 -> RssFile:    {} KB
 -> RssShmem:   {} KB
VmExe:          {} KB
VmLib:          {} KB
VmPTE:          {} KB
HugetlbPages:   {} KB",
            self.status.pid,
            self.status.vm_rss,
            self.status.rss_anon,
            self.status.rss_file,
            self.status.rss_shmem,
            self.status.vm_exe,
            self.status.vm_lib,
            self.status.vm_pte,
            self.status.hugetlb_pages
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