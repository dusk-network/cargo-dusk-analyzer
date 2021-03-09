// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![feature(bool_to_option)]
mod deps;
mod errors;

use cargo_metadata::Dependency;
use console::Style;
use deps::get_invalid_git_dusk_deps;
use tracing::error;

pub use errors::AnalyzeError;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const HEADER: &str = r#"// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.
"#;

pub fn analyze_deps(deps: &[Dependency]) -> Result<(), AnalyzeError> {
    if deps.is_empty() {
        return Ok(());
    }

    let red = Style::new().bright().red();

    let git_dusk_deps = get_invalid_git_dusk_deps(deps);
    let invalid_deps_count = git_dusk_deps.len();
    let has_invalid_deps = invalid_deps_count > 0;

    git_dusk_deps.iter().for_each(|d| {
        error!(
            "{}: {} is a git dependency without a valid tag version specified",
            red.apply_to("error"),
            d.name,
        );
    });

    if has_invalid_deps {
        return Err(AnalyzeError::InvalidDependencies(invalid_deps_count));
    }

    Ok(())
}

pub fn check_license_file(manifest_path: &str) -> Result<(), AnalyzeError> {
    let path = PathBuf::from(manifest_path);
    let path = path.parent().unwrap();

    if !path.join("LICENSE").exists() {
        return Err(AnalyzeError::MissingRootLicense);
    }

    Ok(())
}

pub fn analyze_license(manifest_path: PathBuf) -> Result<(), AnalyzeError> {
    let path = manifest_path.parent();
    let path = path.unwrap();

    let mut failed = 0;
    let mut total = 0;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !path.ends_with("target") && !path.ends_with(".git")
        {
            let (f, t) = check_license_header(&path)?;
            failed += f;
            total += t;
        }
    }

    if failed > 0 {
        let success = total - failed;

        Err(AnalyzeError::MissingLicenseHeader {
            success,
            failed,
            total,
        })
    } else {
        Ok(())
    }
}

fn check_license_header(dir: &Path) -> io::Result<(usize, usize)> {
    let red = Style::new().bright().red();
    let mut failed = 0;
    let mut total = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && !path.ends_with("target") && !path.ends_with(".git")
        {
            let (f, t) = check_license_header(&path)?;
            failed += f;
            total += t;
        } else if let Some(ex) = path.extension() {
            if ex == "rs" {
                total += 1;
                let contents = fs::read_to_string(path)?;

                let is_starting_with_license = contents.starts_with(HEADER);
                let has_only_license =
                    is_starting_with_license && contents.len() == HEADER.len();
                let has_new_line = contents.len() > HEADER.len()
                    && contents.chars().nth(HEADER.len()).unwrap() == '\n';
                let has_license = has_only_license
                    || (is_starting_with_license && has_new_line);

                if !has_license {
                    failed += 1;
                    error!(
                        "{}: {:?} doesn't have the proper license header",
                        red.apply_to("error"),
                        entry.path()
                    );
                }
            }
        }
    }
    Ok((failed, total))
}
