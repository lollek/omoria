use std::process::Command;

fn main() {
    let output = Command::new("date")
        .arg("+%Y%m%d%H%M")
        .output()
        .expect("failed to get the compile timestamp from date");
    assert!(
        output.status.success(),
        "date failed to provide the compile timestamp"
    );

    let timestamp =
        String::from_utf8(output.stdout).expect("date returned a non-UTF-8 compile timestamp");
    let timestamp = timestamp.trim();
    assert_eq!(
        timestamp.len(),
        12,
        "date returned an invalid compile timestamp"
    );
    assert!(
        timestamp.bytes().all(|byte| byte.is_ascii_digit()),
        "date returned an invalid compile timestamp"
    );

    println!("cargo:rustc-env=OMORIA_VERSION=5-alpha.{timestamp}");
    println!("cargo:rerun-if-changed=build.rs");
}
