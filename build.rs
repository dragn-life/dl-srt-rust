/*
 * dl-srt-rust
 * Copyright (C) 2024 DragN Life LLC (Adam B)
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Specify the path to your SRT library directory
    let srt_lib_dir = manifest_dir.join("lib");

    // Tell cargo to look for static libraries in the specified directory
    println!("cargo:rustc-link-search=native={}", srt_lib_dir.display());

    // Link against srt.lib
    println!("cargo:rustc-link-lib=static=srt");
}