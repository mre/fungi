extern crate doxidize;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate tempdir;

mod util;

use doxidize::Config;

use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;

use tempdir::TempDir;

#[test]
fn build_renders_readme() {
    let dir = TempDir::new("create_root_readme").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let config = Config::with_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    let docs_dir = dir_path.join("docs");
    let readme_path = docs_dir.join("README.md");

    let mut readme = OpenOptions::new()
        .create(true)
        .append(true)
        .open(readme_path)
        .expect("could not open README file");

    readme
        .write_all(
            br#"---
id = "readme"
title = "Testing"
---
# Testing

testing"#,
        )
        .expect("could not write to README");

    doxidize::ops::build(&config, &log).expect("build failed");

    let mut output_dir = dir_path.join("target");
    output_dir.push("docs");
    output_dir.push("public");

    let rendered_readme_path = output_dir.join("index.html");

    let mut rendered_readme =
        File::open(rendered_readme_path).expect("could not open rendered README");

    let mut contents = String::new();
    rendered_readme
        .read_to_string(&mut contents)
        .expect("could not read README");

    assert!(contents.contains(
        "<h1>Testing</h1>
<p>testing</p>
"
    ));
}

#[test]
fn build_renders_additional_markdown_files() {
    let dir = TempDir::new("create_additional_markdown").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let config = Config::with_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    let docs_dir = dir_path.join("docs");
    let guide_path = docs_dir.join("guide.md");

    let mut guide = OpenOptions::new()
        .create(true)
        .append(true)
        .open(guide_path)
        .expect("could not open guide file");

    guide
        .write_all(
            br#"---
id = "guide"
title = "Testing"
---
# Testing

testing"#,
        )
        .expect("could not write to guide");

    doxidize::ops::build(&config, &log).expect("generate failed");

    let mut output_dir = dir_path.join("target");
    output_dir.push("docs");
    output_dir.push("public");

    let rendered_guide_path = output_dir.join("guide.html");

    let mut rendered_guide =
        File::open(rendered_guide_path).expect("could not open rendered guide");

    let mut contents = String::new();
    rendered_guide
        .read_to_string(&mut contents)
        .expect("could not read rendered_guide");

    assert!(contents.contains(
        "<h1>Testing</h1>
<p>testing</p>
"
    ));
}

#[test]
fn build_renders_nested_directories() {
    let dir = TempDir::new("create_additional_markdown").expect("could not generate temp dir");
    let log = util::make_logger();

    let dir_path = dir.path();

    util::cargo_init(dir_path).expect("Could not create sample crate");

    let config = Config::with_manifest_path(dir_path.join("Cargo.toml"));

    doxidize::ops::init(&config, &log).expect("init failed");

    let docs_dir = dir_path.join("docs");
    let nested_dir = docs_dir.join("nested");

    fs::create_dir_all(&nested_dir).expect("could not create nested directory");

    let guide_path = nested_dir.join("guide.md");

    let mut guide = OpenOptions::new()
        .create(true)
        .append(true)
        .open(guide_path)
        .expect("could not open guide file");

    guide
        .write_all(
            br#"---
id = "guide"
title = "Testing"
---
# Testing

testing"#,
        )
        .expect("could not write to guide");

    doxidize::ops::build(&config, &log).expect("build failed");

    let mut output_dir = dir_path.join("target");
    output_dir.push("docs");
    output_dir.push("public");

    let mut rendered_guide_path = output_dir.join("nested");
    rendered_guide_path.push("guide.html");

    let mut rendered_guide =
        File::open(rendered_guide_path).expect("could not open rendered guide");

    let mut contents = String::new();
    rendered_guide
        .read_to_string(&mut contents)
        .expect("could not read rendered_guide");

    assert!(contents.contains(
        "<h1>Testing</h1>
<p>testing</p>
"
    ));
}
