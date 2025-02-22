fn main() {
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    println!("cargo:rerun-if-changed=pugl/meson.build");

    if os == "linux" {
        build_linux();
    } else if os == "windows" {
        build_windows();
    } else if os == "macos" {
        build_macos();
    } else {
        panic!("unsupported platform: {}", os);
    }

    // if os == "linux" {
    // } else if os == "windows" {
    //     println!("cargo:rustc-link-search=native=./pugl/build");
    //     println!("cargo:rustc-link-lib=static=pugl_win-0");
    //     println!("cargo:rustc-link-lib=static=pugl_win_stub-0");

    //     if cfg!(feature = "cairo") {
    //         println!("cargo:rustc-link-lib=static=pugl_win_cairo-0");
    //     }
    //     if cfg!(feature = "opengl") {
    //         println!("cargo:rustc-link-lib=static=pugl_win_gl-0");
    //     }
    //     if cfg!(feature = "vulkan") {
    //         println!("cargo:rustc-link-lib=static=pugl_win_vulkan-0");
    //     }
    // } else if os == "macos" {
    //     println!("cargo:rustc-link-search=native=./pugl/build");
    //     println!("cargo:rustc-link-lib=static=pugl_mac-0");
    //     println!("cargo:rustc-link-lib=static=pugl_mac_stub-0");

    //     if cfg!(feature = "cairo") {
    //         println!("cargo:rustc-link-lib=static=pugl_mac_cairo-0");
    //     }
    //     if cfg!(feature = "opengl") {
    //         println!("cargo:rustc-link-lib=static=pugl_mac_gl-0");
    //
    //     }
    //     if cfg!(feature = "vulkan") {
    //         println!("cargo:rustc-link-lib=static=pugl_mac_vulkan-0");
    //     }
    // }
}

fn build_linux() {
    let mut build = cc::Build::new();

    build.include("pugl/include");
    build.file("pugl/src/common.c");
    build.file("pugl/src/internal.c");
    build.file("pugl/src/x11.c");
    build.file("pugl/src/x11_stub.c");

    if cfg!(feature = "cairo") {
        build.file("pugl/src/x11_cairo.c");
    }
    if cfg!(feature = "opengl") {
        build.file("pugl/src/x11_gl.c");
    }
    if cfg!(feature = "vulkan") {
        build.file("pugl/src/x11_vulkan.c");
    }

    build.compile("pugl_x11");

    let mut libs = vec!["X11", "Xext", "Xrandr", "Xcursor"];
    if cfg!(feature = "cairo") {
        libs.push("cairo");
    }
    if cfg!(feature = "opengl") {
        libs.push("GL");
    }
    if cfg!(feature = "vulkan") {
        libs.push("vulkan");
    }

    println!(
        "cargo:rustc-flags={}",
        libs.iter()
            .map(|lib| format!("-l {}", lib))
            .collect::<Vec<_>>()
            .join(" ")
    );
}

fn build_windows() {
    let mut build = cc::Build::new();

    build.include("pugl/include");
    build.file("pugl/src/common.c");
    build.file("pugl/src/internal.c");
    build.file("pugl/src/win.c");
    build.file("pugl/src/win_stub.c");

    if cfg!(feature = "cairo") {
        build.file("pugl/src/win_cairo.c");
    }
    if cfg!(feature = "opengl") {
        build.file("pugl/src/win_gl.c");
    }
    if cfg!(feature = "vulkan") {
        build.file("pugl/src/win_vulkan.c");
    }

    build.compile("pugl_win");

    //TODO: link to the required libraries
}

fn build_macos() {
    let mut build = cc::Build::new();

    build.include("pugl/include");
    build.file("pugl/src/common.c");
    build.file("pugl/src/internal.c");
    build.file("pugl/src/mac.c");
    build.file("pugl/src/mac_stub.c");

    if cfg!(feature = "cairo") {
        build.file("pugl/src/mac_cairo.c");
    }
    if cfg!(feature = "opengl") {
        build.file("pugl/src/mac_gl.c");
    }
    if cfg!(feature = "vulkan") {
        build.file("pugl/src/mac_vulkan.c");
    }

    build.compile("pugl_mac");

    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=CoreVideo");

    if cfg!(feature = "opengl") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
    }

    //TODO: link to the required frameworks
}
