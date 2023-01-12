// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
mod common;
mod proc;
mod mem;

use clap::{Parser, Subcommand};
use common::Inspector;

#[derive(Subcommand, Debug, Clone)]
pub enum InspectCmd {
    /// Show VM's memory consumption.
    Mem {
        /// PID of the hypervisor process.
        #[arg(short, long)]
        pid: u32,
        /// Show the command line arguments.
        #[arg(short, long, default_value_t = false)]
        cmd: bool,
        /// Show the statistics of RSS usage.
        #[arg(short, long, default_value_t = false)]
        status: bool,
        /// Show the detailed RSS usage for each VMA. 
        #[arg(short, long, default_value_t = false)]
        vmas: bool,
    },
}

/// Tool helping to analyse the behaviors for a VM.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct InspectArgs {
    /// Metric to inspect
    #[command(subcommand)]
    pub cmd: InspectCmd,
}

fn main() {
    let args = InspectArgs::parse();
    match &args.cmd {
        InspectCmd::Mem { pid, cmd, status, vmas} => {
            mem::MemInspector::new(*pid, *cmd, *status, *vmas).inspect();
        }
    }
}