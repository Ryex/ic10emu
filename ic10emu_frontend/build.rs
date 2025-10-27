use dotenv::dotenv;

use std::env;

fn main() {
    dotenv().ok();
    println!("cargo::rerun-if-changed=.env");
    println!("cargo::rerun-if-env-changed=SQLD_DB_URL");
    println!("cargo::rerun-if-env-changed=SQLD_AUTH_TOKEN");

    for (name, val) in dotenv::vars() {
        println!("cargo::rustc-env={name}={val}");
    }

    build_data::set_GIT_BRANCH().unwrap();
    build_data::set_GIT_COMMIT().unwrap();
    build_data::set_GIT_COMMIT_SHORT().unwrap();
    build_data::set_RUSTC_VERSION().unwrap();
    build_data::set_RUSTC_VERSION_SEMVER().unwrap();
    build_data::set_GIT_DIRTY().unwrap();

    // build_data::no_debug_rebuilds().unwrap();


}
