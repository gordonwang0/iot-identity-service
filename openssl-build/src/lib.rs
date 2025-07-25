// Copyright (c) Microsoft. All rights reserved.

#![deny(rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_panics_doc, clippy::must_use_candidate)]

/// Emits `ossl110` and `ossl330` cfg based on the version of openssl.
pub fn define_version_number_cfg() {
    let openssl_version = std::env::var("DEP_OPENSSL_VERSION_NUMBER")
        .expect("DEP_OPENSSL_VERSION_NUMBER must have been set by openssl-sys");
    let openssl_version = u64::from_str_radix(&openssl_version, 16)
        .expect("DEP_OPENSSL_VERSION_NUMBER must have been set to a valid integer");

    println!("cargo:rustc-check-cfg=cfg(ossl110, ossl300)");

    #[allow(clippy::unusual_byte_groupings)]
    {
        if openssl_version >= 0x03_00_00_00_0 {
            println!("cargo:rustc-cfg=ossl300");
        }

        if openssl_version >= 0x01_01_00_00_0 {
            println!("cargo:rustc-cfg=ossl110");
        }
    }
}

/// Create an instance of `cc::Build` set up to compile against openssl.
pub fn get_c_compiler() -> cc::Build {
    let openssl_include_path = std::env::var_os("DEP_OPENSSL_INCLUDE")
        .expect("DEP_OPENSSL_INCLUDE must have been set by openssl-sys");

    let mut build = cc::Build::new();
    build.include(openssl_include_path);

    build.warnings_into_errors(true);

    build
}
