use std::process::Command;

fn main() {
    println!(r"cargo:rustc-link-search=target/debug");
    let os = Command::new("uname").output().unwrap();
    let ext = match String::from_utf8_lossy(os.stdout.as_slice())
        .into_owned()
        .trim_end()
        .as_ref()
    {
        "Darwin" => "dylib",
        _ => "so",
    };
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeString.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libDecodeString.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeI8.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libDecodeI8.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeU16.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libDecodeU16.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeU32.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libDecodeU32.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeVecU8.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libDecodeVecU8.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeOptionBool.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libEncodeOptional.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
    Command::new("go")
        .args(&[
            "build",
            "-o",
            &format!("target/debug/libDecodeOptional.{}", ext),
            "-buildmode=c-shared",
            "src/scale.go",
        ])
        .status()
        .unwrap();
}
