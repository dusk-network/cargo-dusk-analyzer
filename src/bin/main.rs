// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use cargo_metadata::MetadataCommand;
use console::style;
use dusk_analyzer::AnalyzeError;
use std::process;

mod args;
mod version;

fn run() -> Result<(), AnalyzeError> {
    let matches = args::matches();

    let manifest_path =
        matches.value_of("manifest-path").unwrap_or("./Cargo.toml");

    let mut cmd_meta = MetadataCommand::new();
    cmd_meta.manifest_path(manifest_path);
    let metadata = cmd_meta.no_deps().exec()?;

    if metadata.packages.is_empty() {
        return Err(AnalyzeError::NoMainPackage);
    }

    for package in metadata.packages {
        let name = package.name;
        let manifest_path = package.manifest_path;
        println!("{:>12} {}", style("Checking").bright().green(), name);

        dusk_analyzer::analyze_deps(&package.dependencies[..])?;
        dusk_analyzer::analyze_license(manifest_path)?;
    }

    Ok(())
}

fn main() {
    let format = tracing_subscriber::fmt::format()
        .without_time()
        .with_target(false)
        .with_level(false)
        .compact();

    tracing_subscriber::fmt().event_format(format).init();

    process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!(" {}: {}", style("error").bright().red(), err);
            1
        }
    });
}
