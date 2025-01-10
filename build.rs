use bindgen::callbacks::{EnumVariantValue, ItemInfo, ItemKind, ParseCallbacks};
use convert_case::{Case, Casing};
use regex::Regex;
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
const USE_EXTERNAL_GLFW: bool = cfg!(feature = "external_glfw");

#[derive(Debug, Clone)]
struct RustyRenamer;

impl RustyRenamer {
    const PREFIXES: &'static [&'static str] = &[
        "LOG_",
        "FLAG_",
        "KEY_",
        "WINDOW_",
        "MATERIAL_MAP_",
        "SHADER_LOC_",
        "SHADER_UNIFORM_",
        "SHADER_ATTRIB_",
        "PIXELFORMAT_",
        "BLEND_",
        "CAMERA_",
        "NPATCH_",
        "PHYSICS_",
    ];
    const SUFFIXES: &'static [&'static str] = &["_HINT", "_WINDOWED_MODE", "_MODE"];
    const RENAMES: &'static [(&'static str, &'static str)] = &[
        ("ConfigFlags", "WindowFlags"),
        ("TraceLogLevel", "LogLevel"),
        ("va_list", "VariadicList"),
        ("__builtin_va_list", "BuiltinVariadicList"),
        ("__va_list_tag", "VaListTag"),
        ("KeyboardKey", "Key"),
    ];
}

impl ParseCallbacks for RustyRenamer {
    fn generated_name_override(&self, _item_info: ItemInfo<'_>) -> Option<String> {
        // don't rename functions starting with __
        match _item_info.kind {
            ItemKind::Function if _item_info.name.starts_with("__") => {
                return None;
            }
            _ => {}
        }
        // rename some specific edge cases
        match _item_info.name {
            "BeginMode2D" => Some("begin_mode_2d".to_string()),
            "EndMode2D" => Some("end_mode_2d".to_string()),
            "BeginMode3D" => Some("begin_mode_3d".to_string()),
            "EndMode3D" => Some("end_mode_3d".to_string()),
            "GetWorldToScreen2D" => Some("get_world_to_screen_2d".to_string()),
            "GetScreenToWorld2D" => Some("get_screen_to_world_2d".to_string()),
            "GetCameraMatrix2D" => Some("get_camera_matrix_2d".to_string()),

            x if x.contains("Vector") => {
                let name = x.to_case(Case::Snake).replace("vector_", "vector");
                Some(name)
            }

            x => Some(x.to_case(Case::Snake)),
        }
    }
    fn enum_variant_name(
        &self,
        _enum_name: Option<&str>,
        _original_variant_name: &str,
        _variant_value: EnumVariantValue,
    ) -> Option<String> {
        let mut name = _original_variant_name;

        for prefix in Self::PREFIXES {
            name = name.trim_start_matches(*prefix);
        }

        for suffix in Self::SUFFIXES {
            name = name.trim_end_matches(*suffix);
        }

        // for the Key enum. Will be converted to PascalCase later
        let name = name.replace("KP", "NUM_PAD");

        // rename some specific edge cases
        match name.as_str() {
            "HIGHDPI" => Some("HighDPI".to_string()),
            "TOPMOST" => Some("AlwaysOnTop".to_string()),

            // convert the case and remove the enum name prefix, if any
            x => Some(
                x.to_case(Case::Pascal)
                    .replace(_enum_name.unwrap_or(""), ""),
            ),
        }
    }

    fn item_name(&self, _original_item_name: &str) -> Option<String> {
        if _original_item_name.contains("PI") {
            return None;
        }

        if _original_item_name.starts_with("rl") {
            return Some(_original_item_name.to_case(Case::Pascal).to_string());
        }

        if _original_item_name.starts_with("PHYSAC_") {
            let underscore_idx = _original_item_name.find('_')?;
            return Some(_original_item_name[underscore_idx + 1..].to_string());
        }

        for (original, renamed) in Self::RENAMES {
            if _original_item_name == *original {
                return Some(renamed.to_string());
            }
        }

        None
    }
}

#[derive(Debug, Clone)]
struct Package<'a> {
    name: &'static str,
    version: &'static str,
    url: &'static str,
    headers: Vec<&'static str>,
    dir: &'a Path,
}

impl Package<'_> {
    fn download(&self) -> anyhow::Result<()> {
        let url = format!("{}/{}.zip", self.url, self.version);
        let path = self.dir.join(self.zip_file_name());
        let mut data = reqwest::blocking::get(&url)?;
        let mut file = File::create(&path)?;

        data.copy_to(&mut file)?;
        file.flush()?;

        Ok(())
    }

    fn extract(&self) -> anyhow::Result<()> {
        let path = self.dir.join(self.zip_file_name());
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

    const WRAPPER_H: &str = "wrapper.h";
    const WRAPPER_C: &str = "wrapper.c";

    let mut file = File::create(dir.join(WRAPPER_H))?;

    for package in &packages {
        for header in &package.headers {
            let header = package.out_dir().join("src").join(header);
            writeln!(file, "#include <{}>", header.display())?;
        }
    }

    file.flush()?;
    std::mem::forget(file);

    let no_copy = vec![
        "Image",
        "(Render)?Texture",
        "GlyphInfo",
        "Font",
        "Mesh",
        "Shader",
        "Material(Map)?",
        "Model(Animation)?",
        "Wave",
        "AudioStream",
        "Sound",
        "Music",
        "VrStereoConfig",
        "FilePathList",
        "AutomationEventList",
        "RlVertexBuffer",
        "RlRenderBatch",
        "PhysicsBodyData",
        "PhysicsManifoldData",
        "PhysicsShape",
    ];

    let no_copy = format!("^({})$", no_copy.join("|"));

    let sys_path = dir.join("..").join("src").join("sys.rs");
    let mut file = File::create(&sys_path)?;
    let wrapper = dir.join(WRAPPER_C);
    let bindings = bindgen::Builder::default()
        .disable_header_comment()
        .rustified_enum(".+")
        .parse_callbacks(Box::new(RustyRenamer))
        .clang_arg("-std=c99")
        .generate_inline_functions(true)
        .header(dir.join(WRAPPER_H).display().to_string())
        .blocklist_function("^__.*")
        .blocklist_item("^(FP|Fp).*")
        .blocklist_item("^(true|false)_")
        .blocklist_item("^__bool_.*")
        .blocklist_type(r"_bindgen_ty_\d+")
        .blocklist_type(r"^__[^bv].*")
        .blocklist_type(r".*?_t$")
        .blocklist_var("^(math|MATH)_.*")
        .blocklist_var("^__glibc.*")
        .blocklist_var(".*?_H$")
        .blocklist_var("^__.*")
        .blocklist_item("^(.*?_)?PI$")
        .no_copy(no_copy)
        .disable_header_comment()
        .layout_tests(false)
        .prepend_enum_name(false)
        .generate()?;

    let mut contents: Vec<u8> = Vec::new();
    writeln!(contents, "#![allow(improper_ctypes)]")?;
    bindings.write(Box::new(&mut contents))?;

    let text = std::str::from_utf8(&contents)?;
    let field_regex = Regex::new(r"pub ([A-Za-z]+): (.*?),")?;
    let contents = field_regex.replace_all(text, |caps: &regex::Captures| {
        let field = caps.get(1).unwrap().as_str();
        let ty = caps.get(2).unwrap().as_str();

        if field.contains("type") {
            format!("pub r#type: {},", ty)
        } else {
            format!("pub {}: {},", field.to_case(Case::Snake), ty)
        }
    });

    file.write_all(contents.as_bytes())?;
    file.flush()?;
    std::mem::forget(file);

    cc::Build::new()
        .include(&dir)
        .file(&wrapper)
        .warnings(false)
        .compile("rraylib");

    let mut config = cmake::Config::new(raylib.out_dir());
    config
        .define("BUILD_EXAMPLES", "OFF")
        .define("INCLUDE_EVERYTHING", "ON")
        .define("OPENGL_VERSION", "OFF")
        .define(
            "CMAKE_BUILD_TYPE",
            if IS_DEBUG { "Debug" } else { "Release" },
        );

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

    if USE_WAYLAND && !USE_X11 && !USE_SDL {
        config.define("PLATFORM", "Desktop");
        config.define("GLFW_BUILD_WAYLAND", "ON");
    }

    if USE_X11 && !USE_WAYLAND && !USE_SDL {
        config
            .define("PLATFORM", "Desktop")
            .define("GLFW_BUILD_X11", "ON");
    }

    if USE_SDL && !USE_WAYLAND && !USE_X11 {
        config
            .define("PLATFORM", "SDL")
            .define("OPENGL_VERSION", "OFF");
    }

    if USE_EXTERNAL_GLFW {
        config.define("USE_EXTERNAL_GLFW", "ON");
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
