//
//  build.rs
//  os_log
//
//  Created by Søren Mortensen on 28/07/2018.
//  Copyright © 2018 Søren Mortensen. All rights reserved.
//

extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let wrapper_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../os_log_wrapper");

    Command::new("make")
        .current_dir(PathBuf::from(wrapper_dir))
        .status().unwrap();
    
    println!("cargo:rustc-link-search=native=../os_log_wrapper/out/");
    println!("cargo:rustc-link-lib=static=oslogwrapper");
    
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/os_log.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("os_log.rs"))
        .expect("Couldn't write bindings!");
}
