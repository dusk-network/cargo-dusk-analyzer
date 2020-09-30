// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::errors::AnalyzeError;
use cargo_metadata::{Dependency, Metadata};
use semver::Version;
use url::Url;

pub fn get_main_package(metadata: &Metadata) -> Result<String, AnalyzeError> {
    if !metadata.packages.is_empty() {
        Ok(metadata.packages[0].name.clone())
    } else {
        Err(AnalyzeError::NoMainPackage)
    }
}

pub fn get_invalid_git_dusk_deps(deps: &[Dependency]) -> Vec<&Dependency> {
    let deps: Vec<&Dependency> = deps
        .iter()
        .filter(|d| d.req.to_string() == "*")
        .filter(|d| {
            d.source
                .as_ref()
                .filter(|s| {
                    s.starts_with("git+https://github.com/dusk-network/")
                })
                .and_then(|s| Url::parse(s).ok())
                .map(|u| {
                    get_tag_version_from_query(u.query())
                        .and_then(|v| Version::parse(v).ok())
                        .map_or(true, |_| false)
                })
                .unwrap_or(false)
        })
        .collect();
    deps
}

fn get_tag_version_from_query(query: Option<&str>) -> Option<&str> {
    match query {
        Some(q) => match q.strip_prefix("tag=") {
            Some(ver) => ver
                .strip_prefix("v.")
                .or_else(|| ver.strip_prefix("v"))
                .or(Some(ver)),
            _ => None,
        },
        _ => None,
    }
}
