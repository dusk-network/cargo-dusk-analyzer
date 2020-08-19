// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyzeError {
    #[error(transparent)]
    MetadataError(#[from] cargo_metadata::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("No main package defined in the metadata")]
    NoMainPackage,
    #[error("Invalid Dependencies (expected 0, found {0})")]
    InvalidDependencies(usize),
    #[error("Missing LICENSE File in the root")]
    MissingRootLicense,
    #[error("Missing license header on {failed} source file(s) [{success}/{total} OK]")]
    MissingLicenseHeader {
        success: usize,
        failed: usize,
        total: usize,
    },
}
