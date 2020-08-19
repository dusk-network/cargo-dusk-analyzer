// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

const BIN_NAME: &str = "cargo";
const COMMAND_DESCRIPTION: &str =
    "A third-party cargo extension for additional code lint & checks";
const COMMAND_AUTHOR: &str = "zer0 <matteo@dusk-network.com>";

use crate::version::show_version;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use rustc_tools_util::*;

pub fn matches() -> ArgMatches<'static> {
    let info = rustc_tools_util::get_version_info!();
    let command_name = &info.crate_name[format!("{}-", BIN_NAME).len()..];

    App::new(&info.crate_name)
        .about(COMMAND_DESCRIPTION)
        .author(COMMAND_AUTHOR)
        .version(show_version(&info).as_str())
        // We have to lie about our binary name since this will be a third party
        // subcommand for cargo, this trick learned from cargo-outdated
        .bin_name(BIN_NAME)
        // We use a subcommand because parsed after `cargo` is sent to the third
        // party plugin which will be interpreted as a subcommand/positional arg
        // by clap
        .subcommand(
            SubCommand::with_name(command_name)
                .about(COMMAND_DESCRIPTION)
                .arg(
                    Arg::with_name("manifest-path")
                        .long("manifest-path")
                        .value_name("PATH")
                        .takes_value(true)
                        .help("Path to Cargo.toml"),
                ),
        )
        .settings(&[AppSettings::SubcommandRequired])
        .get_matches()
        .subcommand_matches(command_name)
        .expect("Cargo subcommand should always match")
        .to_owned()
}
