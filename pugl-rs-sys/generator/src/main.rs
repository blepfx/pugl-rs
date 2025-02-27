use std::{
    env::set_current_dir,
    path::{Path, PathBuf},
};

fn main() {
    set_current_dir(workspace_dir().join("pugl-rs-sys")).unwrap();

    bindings("./pugl/include/pugl/pugl.h", "pugl.rs", false);
    bindings("./pugl/include/pugl/gl.h", "gl.rs", true);
    bindings("./pugl/include/pugl/cairo.h", "cairo.rs", true);
    bindings("./pugl/include/pugl/vulkan.h", "vulkan.rs", true);
    bindings("./pugl/include/pugl/stub.h", "stub.rs", true);

    println!("OK")
}

fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

fn bindings(source: &str, target: &str, dependent: bool) {
    let bindings = if dependent {
        format!(
            "use crate::*;\n{}",
            bindgen::Builder::default()
                .header(source)
                .allowlist_file(source)
                .blocklist_type("PuglWorldImpl")
                .blocklist_type("PuglWorld")
                .blocklist_type("PuglViewImpl")
                .blocklist_type("PuglView")
                .blocklist_type("PuglBackendImpl")
                .blocklist_type("PuglBackend")
                .blocklist_type("PuglStatus")
                .layout_tests(false)
                .prepend_enum_name(false)
                .clang_arg("-Ipugl/include")
                .generate()
                .expect("Unable to generate bindings")
                .to_string()
        )
    } else {
        bindgen::Builder::default()
            .header(source)
            .allowlist_file(source)
            .layout_tests(false)
            .prepend_enum_name(false)
            .clang_arg("-Ipugl/include")
            .generate()
            .expect("Unable to generate bindings")
            .to_string()
    };

    std::fs::write(
        std::path::PathBuf::from(format!("./src/generated/{}", target)),
        bindings,
    )
    .expect("Unable to write bindings");
}
