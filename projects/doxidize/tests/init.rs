extern crate doxidize;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate tempdir;

use doxidize::Config;
use tempdir::TempDir;

mod util;

#[test]
fn creates_docs_dir() {
    let dir = TempDir::new("create_docs_dir").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let mut config = Config::default();
    config.set_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    assert!(dir_path.join("docs").is_dir());
}

#[test]
fn creates_root_readme() {
    let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let mut config = Config::default();
    config.set_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    let docs_dir = dir_path.join("docs");
    let readme_path = docs_dir.join("README.md");

    assert!(readme_path.is_file());
}

#[test]
fn creates_doxidize_config() {
    let dir = TempDir::new("create_doxidize_config").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let mut config = Config::default();
    config.set_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    let config_path = dir_path.join("Doxidize.toml");

    assert!(config_path.is_file());
}

#[test]
fn double_initialize() {
    let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let mut config = Config::default();
    config.set_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    doxidize::ops::init(&config, &log).expect("init failed when run a second time");
}

#[test]
fn creates_menu_toml() {
    let dir = TempDir::new("create_menu_toml").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let mut config = Config::default();
    config.set_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    let docs_dir = dir_path.join("docs");
    let readme_path = docs_dir.join("Menu.toml");

    assert!(readme_path.is_file());
}
