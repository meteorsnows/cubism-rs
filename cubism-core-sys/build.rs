use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut target_s = target.split('-');
    let (arch, vendor, sys, _abi) = (
        target_s.next().unwrap(),
        target_s.next().unwrap(),
        target_s.next().unwrap(),
        target_s.next().unwrap_or(""),
    );
    let mut lib_dir = match env::var("CUBISM_CORE") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => panic!(
            "The CUBISM_CORE environment variable hasn't been set! \
             Please set it to your Live2DCubismCore directory before compiling. \
             See the readme for more information."
        ),
    };
    lib_dir.push("lib");
    match vendor {
        "pc" => {
            //windows
            lib_dir.push(sys);
            if arch == "x86_64" {
                lib_dir.push("x86_64");
            } else {
                lib_dir.push("x86");
            }
        }
        "apple" => {
            lib_dir.push(match sys {
                "ios" => "ios",
                _ => "macos",
            });
        }
        _ => match sys {
            "android" => lib_dir.push(match arch {
                "i686" => "x86",
                "armv7" => "armeabi-v7a",
                _ => "arm64-v8a",
            }),
            _ => lib_dir.push("linux/x86_64"),
        },
    }
    println!("cargo:rustc-link-search=all={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=Live2DCubismCore");
}
