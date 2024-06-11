use std::{path::PathBuf, str::FromStr};

// macro_rules! build_println {
//     ($($tokens: tt)*) => {
//         println!("cargo:warning={}", format!($($tokens)*))
//     }
// }

fn main() {
    let android_home = std::env::var("ANDROID_HOME").expect("ANDROID_HOME not set");
    let option_ndk_home = std::env::var("NDK_HOME").ok();

    let ndk_root = match option_ndk_home {
        Some(ndk_root) => PathBuf::from_str(&ndk_root).expect("Failed to create NDK path"),
        None => {
            let ndk_entry = format!("{}/ndk", android_home);
            let ndk_entry = PathBuf::from_str(&ndk_entry).expect("Failed to create NDK path");
            let folders = ndk_entry.read_dir().expect("Failed to read NDK folder");
            let mut folders = folders
                .into_iter()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();

            folders.sort_by(|a, b| b.cmp(a));

            let first = folders.first().expect("No NDK folder found");

            first.clone()
        }
    };

    let host_platform = std::env::consts::OS;

    let host = match host_platform {
        "linux" => "linux-x86_64",
        "macos" => "darwin-x86_64",
        "windows" => "windows-x86_64",
        _ => panic!("Unsupported host platform: {}", host_platform),
    };

    let includes = ndk_root.join(format!(
        "toolchains/llvm/prebuilt/{}/sysroot/usr/include",
        host
    ));

    if !includes.exists() {
        panic!(
            "NeuralNetworks.h or NeuralNetworksTypes.h not found in NDK folder: {}",
            ndk_root.to_str().unwrap()
        );
    }

    #[cfg(feature = "api-level-33")]
    let api_level = "33";

    let lib_dir = ndk_root.join(format!(
        "toolchains/llvm/prebuilt/{}/sysroot/usr/lib/aarch64-linux-android/{}",
        host, api_level
    ));

    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=neuralnetworks");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", includes.join("android").to_str().unwrap()))
        .clang_arg(format!("-I{}", includes.to_str().unwrap()))
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
