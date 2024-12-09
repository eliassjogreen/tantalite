use std::{
    env::{self, current_dir},
    path::PathBuf,
    process::Command,
};

fn build_dir() -> PathBuf {
    let cwd = current_dir().unwrap();

    let out_dir = env::var_os("OUT_DIR").expect(
        "The 'OUT_DIR' environment is not set (it should be something like \
       'target/debug').",
    );
    let out_dir_abs = cwd.join(out_dir);

    // This would be target/debug or target/release
    out_dir_abs
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn main() {
    let sqlite_version = env::var_os("SQLITE_VERSION")
        .unwrap_or("3470200".into())
        .into_string()
        .unwrap();
    let sqlite_amalgamation_zip = build_dir()
        .join(format!("sqlite-amalgamation-{sqlite_version}.zip"))
        .to_str()
        .unwrap()
        .to_owned();
    let sqlite_amalgamation_url =
        format!("https://www.sqlite.org/2024/sqlite-amalgamation-{sqlite_version}.zip");

    Command::new("curl")
        .arg(sqlite_amalgamation_url)
        .args(["-o", &sqlite_amalgamation_zip])
        .status()
        .expect("Failed to download SQLite amalgamation");

    Command::new("unzip")
        .arg(&sqlite_amalgamation_zip)
        .args(["-d", build_dir().to_str().unwrap()])
        .status()
        .expect("Failed to unzip SQLite amalgamation");
}
