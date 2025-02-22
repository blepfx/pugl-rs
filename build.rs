use std::process::Command;

fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let profile = std::env::var("PROFILE").unwrap();

    Command::new("meson")
        .current_dir("pugl")
        .args([
            "setup",
            "build",
            "--reconfigure",
            "--buildtype",
            if profile == "release" {
                "release"
            } else {
                "debug"
            },
            &format!(
                "-Dopengl={}",
                if cfg!(feature = "opengl") {
                    "enabled"
                } else {
                    "disabled"
                }
            ),
            &format!(
                "-Dvulkan={}",
                if cfg!(feature = "vulkan") {
                    "enabled"
                } else {
                    "disabled"
                }
            ),
            &format!(
                "-Dcairo={}",
                if cfg!(feature = "cairo") {
                    "enabled"
                } else {
                    "disabled"
                }
            ),
            "-Dexamples=disabled",
            "-Dtests=disabled",
            "-Dbindings_cpp=disabled",
            "-Ddefault_library=static",
        ])
        .status()
        .expect("pugl: meson build failed");

    Command::new("meson")
        .arg("compile")
        .current_dir("pugl/build")
        .status()
        .expect("pugl: meson compile failed");

    let mut libs = vec![];

    if os == "linux" {
        libs.extend(["X11", "Xext", "Xrandr", "Xcursor"]);

        if cfg!(feature = "opengl") {
            libs.push("GL");
        }

        if cfg!(feature = "cairo") {
            libs.push("cairo");
        }

        if cfg!(feature = "vulkan") {
            libs.push("vulkan");
        }
    } else if os == "windows" {
        libs.extend(["shlwapi", "dwmapi", "user32", "gdi32"]);

        if cfg!(feature = "cairo") {
            libs.push("cairo");
        }

        if cfg!(feature = "vulkan") {
            libs.push("vulkan");
        }
    } else if os == "macos" {
        libs.extend(["CoreFoundation", "CoreVideo"]);

        if cfg!(feature = "opengl") {
            libs.push("GL");
        }

        if cfg!(feature = "cairo") {
            libs.push("cairo");
        }

        if cfg!(feature = "vulkan") {
            libs.push("vulkan");
        }
    } else {
        panic!("unsupported target os: only linux, windows and macos are supported")
    };

    if os == "linux" {
        println!("cargo:rustc-link-search=native=./pugl/build");
        println!("cargo:rustc-link-lib=static=pugl_x11-0");
        println!("cargo:rustc-link-lib=static=pugl_x11_stub-0");

        if cfg!(feature = "cairo") {
            println!("cargo:rustc-link-lib=static=pugl_x11_cairo-0");
        }
        if cfg!(feature = "opengl") {
            println!("cargo:rustc-link-lib=static=pugl_x11_gl-0");
        }
        if cfg!(feature = "vulkan") {
            println!("cargo:rustc-link-lib=static=pugl_x11_vulkan-0");
        }
    } else if os == "windows" {
        println!("cargo:rustc-link-search=native=./pugl/build");
        println!("cargo:rustc-link-lib=static=pugl_win-0");
        println!("cargo:rustc-link-lib=static=pugl_win_stub-0");

        if cfg!(feature = "cairo") {
            println!("cargo:rustc-link-lib=static=pugl_win_cairo-0");
        }
        if cfg!(feature = "opengl") {
            println!("cargo:rustc-link-lib=static=pugl_win_gl-0");
        }
        if cfg!(feature = "vulkan") {
            println!("cargo:rustc-link-lib=static=pugl_win_vulkan-0");
        }
    } else if os == "macos" {
        println!("cargo:rustc-link-search=native=./pugl/build");
        println!("cargo:rustc-link-lib=static=pugl_mac-0");
        println!("cargo:rustc-link-lib=static=pugl_mac_stub-0");

        if cfg!(feature = "cairo") {
            println!("cargo:rustc-link-lib=static=pugl_mac_cairo-0");
        }
        if cfg!(feature = "opengl") {
            println!("cargo:rustc-link-lib=static=pugl_mac_gl-0");
        }
        if cfg!(feature = "vulkan") {
            println!("cargo:rustc-link-lib=static=pugl_mac_vulkan-0");
        }
    }

    println!(
        "cargo:rustc-flags={}",
        libs.iter()
            .map(|lib| format!("-l {}", lib))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
