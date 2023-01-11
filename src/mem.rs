// Copyright 2018 Amazon.com, Inc. or its affiliates. All Right&s Reserved.
// SPDX-License-Identifier: Apache-2.0
use super::common::{Inspector};
use super::proc::*;

/// Inspector for VM's memory consumption.  
pub struct MemInspector {
    /// Command line parameters.
    cmd: Option<ProcCmd>,
    /// Memory status for a process.
    status: Option<ProcStatus>,
    /// VMAs for a Process.
    smaps: Option<ProcSmaps>,
}

impl Inspector for MemInspector {
    fn inspect(&self) {
        if self.cmd.is_some() {
            println!("{}", self.cmd.as_ref().unwrap());
        }
        if self.status.is_some() {
            println!("{}", self.status.as_ref().unwrap());
        }
        if self.smaps.is_some() {
            println!("{}", self.smaps.as_ref().unwrap());
        }
    }
}

impl MemInspector {
    pub fn new(pid: u32, cmd: bool, status: bool, smaps: bool) -> Self {
        MemInspector {
            cmd: if cmd { Some(proc_pid_cmdline(pid).unwrap()) } else { None },
            status: if status { Some(proc_pid_status(pid).unwrap()) } else { None },
            smaps: if smaps { Some(proc_pid_smaps(pid).unwrap()) } else { None }
        }
    }
}
