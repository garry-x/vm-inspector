// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
mod common;
mod proc;
mod mem;

use clap::{Parser};

use common::{InspectArgs, InspectCmd, Inspector};

fn main() {
    let args = InspectArgs::parse();
    match &args.cmd {
        InspectCmd::Mem { pid, cmd, status, vmas} => {
            mem::MemInspector::new(*pid, *cmd, *status, *vmas).inspect();
        }
    }
}