//! This tiny build script ensures that rocket is not compiled with an
//! incompatible version of rust.

extern crate version_check;

use version_check::{supports_features, is_min_version, is_min_date};

// Specifies the minimum nightly version needed to compile Rocket.
const MIN_DATE: &'static str = "2018-11-23";
const MIN_VERSION: &'static str = "1.32.0-nightly";

fn main() {
    let ok_channel = supports_features();
    let ok_version = is_min_version(MIN_VERSION);
    let ok_date = is_min_date(MIN_DATE);
    let triple = (ok_channel, ok_version, ok_date);

    let print_version_err = |version: &str, date: &str| {
        eprintln!("Installed version is: {} {}. Minimum required: {} {}.", version, date, MIN_VERSION, MIN_DATE);
    };

    if let (Some(ok_channel), Some((ok_version, version)), Some((ok_date, date))) = triple {
        if !ok_channel {
            eprintln!("Error: Rocket requires a nightly or dev version of Rust.");
            print_version_err(&*version, &*date);
            eprintln!("See the getting started guide (https://rocket.rs/v0.4/guide/getting-started/) for more information.");
            panic!("Aborting compilation due to incompatible compiler.")
        }

        if !ok_version || !ok_date {
            eprintln!("Error: Rocket requires a more recent version of rustc.");
            eprintln!("Use `rustup update` or your preferred method to update Rust.");
            print_version_err(&*version, &*date);
            panic!("Aborting compilation due to incompatible compiler.")
        }
    } else {
        println!("cargo:warning={}", "Rocket was unable to check rustc compatibility.");
        println!("cargo:warning={}", "Build may fail due to incompatible rustc version.");
    }
}
