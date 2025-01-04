use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const IS_DEBUG: bool = cfg!(debug_assertions);
const USE_WAYLAND: bool = cfg!(feature = "wayland");
const USE_X11: bool = cfg!(feature = "x11");
const USE_OPENGL_43: bool = cfg!(feature = "opengl_43");
const USE_OPENGL_33: bool = cfg!(feature = "opengl_33");
const USE_OPENGL_21: bool = cfg!(feature = "opengl_21");
const USE_OPENGL_11: bool = cfg!(feature = "opengl_11");
const USE_OPENGL_ES_30: bool = cfg!(feature = "opengl_es_30");
const USE_OPENGL_ES_20: bool = cfg!(feature = "opengl_es_20");
const USE_GUI: bool = cfg!(feature = "raygui");
const USE_PHYSAC: bool = cfg!(feature = "physac");
const USE_SDL: bool = cfg!(feature = "sdl");

#[derive(Debug, Clone)]
struct Package<'a> {
    name: &'static str,
    version: &'static str,
    url: &'static str,
    headers: Vec<&'static str>,
    dir: &'a Path,
}

impl<'a> Package<'a> {
    fn download(&self) -> anyhow::Result<()> {
        let url = format!("{}/{}.zip", self.url, self.version);
        let path = PathBuf::from(self.dir.join(self.zip_file_name()));
        let mut data = reqwest::blocking::get(&url)?;
        let mut file = File::create(&path)?;

        data.copy_to(&mut file)?;
        file.flush()?;

        Ok(())
    }

    fn extract(&self) -> anyhow::Result<()> {
        let path = PathBuf::from(self.dir.join(self.zip_file_name()));
        let mut file = File::open(&path)?;

        zip_extract::extract(&mut file, self.dir, false)?;

        Ok(())
    }

    fn zip_file_name(&self) -> String {
        format!("{}.zip", self.name)
    }

    fn out_dir(&self) -> PathBuf {
        self.dir.join(format!("{}-{}", self.name, self.version))
    }
}

fn main() -> anyhow::Result<()> {
    let dir = std::env::current_dir()?.join("vendor");
    std::fs::create_dir_all(&dir)?;

    let raylib = Package {
        name: "raylib",
        version: "5.5",
        url: "https://github.com/raysan5/raylib/archive/refs/tags",
        headers: vec!["raylib.h", "rlgl.h", "raymath.h"],
        dir: &dir,
    };

    let raygui = Package {
        name: "raygui",
        version: "4.0",
        url: "https://github.com/raysan5/raygui/archive/refs/tags",
        headers: vec!["raygui.h"],
        dir: &dir,
    };

    let physac = Package {
        name: "Physac",
        version: "1.1",
        url: "https://github.com/victorfisac/Physac/archive/refs/tags",
        headers: vec!["physac.h"],
        dir: &dir,
    };

    let mut packages = vec![raylib.clone()];

    if USE_GUI {
        packages.push(raygui);
    }

    if USE_PHYSAC {
        packages.push(physac);
    }

    for package in &packages {
        package.download()?;
        package.extract()?;
    }

    const WRAPPER_H: &'static str = "wrapper.h";
    const WRAPPER_C: &'static str = "wrapper.c";

    let mut file = File::create(dir.join(WRAPPER_H))?;

    for package in &packages {
        for header in &package.headers {
            let header = package.out_dir().join("src").join(header);
            writeln!(file, "#include <{}>", header.display())?;
        }
    }

    file.flush()?;
    std::mem::forget(file);

    let sys_path = dir.join("..").join("src").join("sys.rs");
    let mut file = File::create(&sys_path)?;
    let wrapper = dir.join(WRAPPER_C);
    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .rustified_enum(".+")
        .clang_arg("-std=c99")
        .generate_inline_functions(true)
        .header(dir.join(WRAPPER_H).display().to_string())
        .blocklist_item("FP_.+")
        .blocklist_type("(true|false)_")
        .blocklist_function("__bool.+")
        .prepend_enum_name(false)
        .generate()?;

    bindings.write(Box::new(&file))?;
    file.flush()?;
    std::mem::forget(file);

    cc::Build::new()
        .include(&dir)
        .file(&wrapper)
        .compile("rraylib");

    let mut config = cmake::Config::new(raylib.out_dir());
    config
        .define("BUILD_EXAMPLES", "OFF")
        .define("INCLUDE_EVERYTHING", "ON")
        .define(
            "CMAKE_BUILD_TYPE",
            if IS_DEBUG { "Debug" } else { "Release" },
        );

    if USE_WAYLAND && !USE_X11 && !USE_SDL {
        config.define("PLATFORM", "Desktop");
        config.define("GLFW_BUILD_WAYLAND", "ON");
    }

    if USE_X11 && !USE_WAYLAND && !USE_SDL {
        config.define("PLATFORM", "Desktop");
        config.define("GLFW_BUILD_X11", "ON");
    }

    if USE_SDL && !USE_WAYLAND && !USE_X11 {
        config.define("PLATFORM", "SDL");
        config.define("USE_EXTERNAL_GLFW", "ON");
    }

    if USE_OPENGL_43 {
        config.define("OPENGL_VERSION", "4.3");
    }

    if USE_OPENGL_33 {
        config.define("OPENGL_VERSION", "3.3");
    }

    if USE_OPENGL_21 {
        config.define("OPENGL_VERSION", "2.1");
    }

    if USE_OPENGL_11 {
        config.define("OPENGL_VERSION", "1.1");
    }

    if USE_OPENGL_ES_30 {
        config.define("OPENGL_ES_VERSION", "ES 3.0");
    }

    if USE_OPENGL_ES_20 {
        config.define("OPENGL_ES_VERSION", "ES 2.0");
    }

    let out_dir = config.build();
    let search_dirs = vec!["lib64", "lib", "bin"];
    for search_dir in search_dirs {
        let search_dir = out_dir.join(search_dir);
        println!(
            "cargo:rustc-link-search=native={}",
            raylib.out_dir().join(search_dir).display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=static=raylib");
    println!("cargo:rustc-link-lib=static=rraylib");

    Ok(())
}
