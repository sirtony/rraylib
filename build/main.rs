use crate::pkg::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod pkg;

fn main() -> anyhow::Result<()> {
    let workdir = tempfile::tempdir()?;
    let raylib_out = RAYLIB.download(&workdir)?;
    let raygui_out = RAYGUI.download(&workdir)?;
    let physac_out = PHYSAC.download(&workdir)?;
    let raylib_src = raylib_out.join("src");
    let raygui_src = raygui_out.join("src");
    let physac_src = physac_out.join("src");
    let mut bindings = bindgen::Builder::default()
        .disable_header_comment()
        .clang_arg(format!("-I{}", raylib_src.display()))
        .clang_arg(format!("-I{}", raygui_src.display()))
        .clang_arg(format!("-I{}", physac_src.display()))
        .clang_arg("-std=c99")
        .rustified_enum(".+")
        .blocklist_function("^__.*")
        .blocklist_function("^[a-z0-9]+$")
        .blocklist_var("^__.*")
        .blocklist_var("RL_ZERO|RL_ONE")
        .blocklist_var("^(true|false)_")
        .blocklist_var("^.*?_VERSION.*$")
        .blocklist_var("^.*?_?(DEG2RAD|RAD2DEG|PI|MIN|MAX|EPSILON)$")
        .blocklist_var("^FP_(NAN|INFINITE|ZERO|SUBNORMAL|NORMAL)$")
        .blocklist_type("^va_.*")
        .blocklist_var("^FP_.*")
        .blocklist_var(".*_H$")
        .blocklist_var("^(MATH|math)_.*$")
        .blocklist_type("_bindgen_ty_1")
        .blocklist_type("^__[^v].*")
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate_comments(true)
        .detect_include_paths(true)
        .generate_inline_functions(true)
        .header(raylib_src.join("raylib.h").display().to_string())
        .header(raylib_src.join("raymath.h").display().to_string())
        .header(raylib_src.join("rlgl.h").display().to_string());

    if cfg!(feature = "raygui") {
        bindings = bindings.header(raygui_src.join("raygui.h").display().to_string());
    }

    if cfg!(feature = "physac") {
        bindings = bindings.header(physac_src.join("physac.h").display().to_string());
    }

    let bindings = bindings.generate()?;

    let mut file = File::create(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("sys.rs"),
    )?;

    writeln!(&mut file, "#![allow(non_camel_case_types)]")?;
    writeln!(&mut file, "#![allow(non_snake_case)]")?;
    writeln!(&mut file, "#![allow(dead_code)]")?;
    bindings.write(Box::new(&mut file))?;

    file.flush()?;
    file.sync_all()?;

    let mut config = cmake::Config::new(&raylib_out);
    config
        .define("BUILD_EXAMPLES", "OFF")
        .define("OPENGL_VERSION", "OFF")
        .define(
            "CMAKE_BUILD_TYPE",
            if cfg!(debug_assertions) {
                "Debug"
            } else {
                "Release"
            },
        );

    if cfg!(feature = "opengl_43") {
        config.define("OPENGL_VERSION", "4.3");
    }

    if cfg!(feature = "opengl_33") {
        config.define("OPENGL_VERSION", "3.3");
    }

    if cfg!(feature = "opengl_21") {
        config.define("OPENGL_VERSION", "2.1");
    }

    if cfg!(feature = "opengl_11") {
        config.define("OPENGL_VERSION", "1.1");
    }

    if cfg!(feature = "opengl_es_30") {
        config.define("OPENGL_ES_VERSION", "ES 3.0");
    }

    if cfg!(feature = "opengl_es_20") {
        config.define("OPENGL_ES_VERSION", "ES 2.0");
    }

    if cfg!(feature = "wayland") && !cfg!(feature = "x11") && !cfg!(feature = "sdl") {
        config.define("PLATFORM", "Desktop");
        config.define("GLFW_BUILD_WAYLAND", "ON");
    }

    if cfg!(feature = "x11") && !cfg!(feature = "wayland") && !cfg!(feature = "sdl") {
        config
            .define("PLATFORM", "Desktop")
            .define("GLFW_BUILD_X11", "ON");
    }

    if cfg!(feature = "sdl") && !cfg!(feature = "wayland") && !cfg!(feature = "x11") {
        config
            .define("PLATFORM", "SDL")
            .define("OPENGL_VERSION", "OFF");
    }

    if cfg!(feature = "external_glfw") {
        config.define("USE_EXTERNAL_GLFW", "ON");
    }

    let out_dir = config.build();
    let search_dirs = vec!["lib64", "lib32", "lib", "bin"];
    for search_dir in search_dirs {
        let search_dir = out_dir.join(search_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            raylib_out.join(search_dir).display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=static=raylib");

    if cfg!(feature = "raygui") {
        cc::Build::new()
            .include(&raylib_src)
            .include(raygui_src)
            .warnings(false)
            .file(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("build")
                    .join("raygui.c"),
            )
            .compile("raygui");
        println!("cargo:rustc-link-lib=static=raygui");
    }

    if cfg!(feature = "physac") {
        cc::Build::new()
            .include(raylib_src)
            .include(physac_src)
            .warnings(false)
            .file(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("build")
                    .join("physac.c"),
            )
            .compile("physac");
        println!("cargo:rustc-link-lib=static=physac");
    }

    Ok(())
}
