use crate::command;
use std::{fs, io::ErrorKind};

fn install_rust_android_gradle() {
    const PLUGIN_COMMIT: &str = "6e553c13ef2d9bb40b58a7675b96e0757d1b0443";
    const PLUGIN_VERSION: &str = "0.8.3";

    let rust_android_archive_url = format!(
        "https://github.com/mozilla/rust-android-gradle/archive/{}.zip",
        PLUGIN_COMMIT
    );

    let download_path = cached_path::cached_path_with_options(
        &rust_android_archive_url,
        &cached_path::Options::default().extract(),
    )
    .unwrap();
    let download_path = download_path.join(format!("rust-android-gradle-{}", PLUGIN_COMMIT));

    #[cfg(windows)]
    let gradlew_path = download_path.join("gradlew.bat");
    #[cfg(target_os = "linux")]
    let gradlew_path = download_path.join("gradlew");

    command::run_in(
        &download_path,
        &format!("{} publish", gradlew_path.to_string_lossy()),
    )
    .unwrap();

    let dep_dir = crate::workspace_dir()
        .join("deps")
        .join("rust-android-gradle");
    if let Err(e) = fs::create_dir_all(&dep_dir) {
        if e.kind() != ErrorKind::AlreadyExists {
            panic!(e);
        }
    }

    // Workaround for long path issue on Windows - canonicalize
    let plugin_path = download_path.canonicalize().unwrap();
    let plugin_path = plugin_path
        .join("samples")
        .join("maven-repo")
        .join("org")
        .join("mozilla")
        .join("rust-android-gradle")
        .join("rust-android")
        .join(PLUGIN_VERSION)
        .join(format!("rust-android-{}.jar", PLUGIN_VERSION));
    fs::copy(
        plugin_path,
        dep_dir.join(format!("rust-android-{}.jar", PLUGIN_VERSION)),
    )
    .unwrap();
}

fn build_ffmpeg() {
    let download_path = cached_path::cached_path_with_options(
        &format!(
            "https://git.ffmpeg.org/gitweb/ffmpeg.git/snapshot/{}.tar.gz",
            "f719f869907764e6412a6af6e178c46e5f915d25"
        ),
        &cached_path::Options::default().extract(),
    )
    .unwrap();
    let download_path = download_path.join("ffmpeg-f719f86");

    command::run_as_bash("sudo apt update").unwrap();
}

pub fn install_deps() {
    command::run("rustup target add aarch64-linux-android").unwrap();
    install_rust_android_gradle();
    build_ffmpeg();
}
