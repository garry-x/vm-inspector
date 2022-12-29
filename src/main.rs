// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
mod common;
mod mem;

use clap::{Parser, Subcommand};
use common::Inspector;

#[derive(Subcommand, Debug)]
enum InspectCmd {
    /// Show VM's memory consumption
    Mem {
        /// PID of the target VM
        #[arg(short, long)]
        pid: u32,
    }
}

/// Tool helping to analyse the behaviors for a VM.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct InspectArgs {
    /// Metric to inspect
    #[command(subcommand)]
    cmd: InspectCmd,
}

fn main() {
    let args = InspectArgs::parse();
    match &args.cmd {
        InspectCmd::Mem { pid } => {
            mem::MemInspector::new(*pid).inspect();
        }
    }
}