use std::io::{BufReader, Seek, SeekFrom, Write};
use std::path::PathBuf;
use tempfile::TempDir;

pub struct Pkg {
    pub dev: &'static str,
    pub name: &'static str,
    pub version: &'static str,
}

impl Pkg {
    pub fn download(&self, workdir: &TempDir) -> anyhow::Result<PathBuf> {
        let url = self.release_url();
        let response = reqwest::blocking::get(&url)?;
        let bytes = response.bytes()?;
        let mut file = tempfile::tempfile()?;

        file.write_all(&bytes)?;
        file.flush()?;
        file.seek(SeekFrom::Start(0))?;

        let reader = BufReader::new(file);
        zip_extract::extract(reader, workdir.as_ref(), false)?;

        let path = workdir
            .path()
            .to_path_buf()
            .join(format!("{}-{}", self.name, self.version));

        Ok(path)
    }

    fn release_url(&self) -> String {
        format!(
            "https://github.com/{}/{}/archive/refs/tags/{}.zip",
            self.dev, self.name, self.version
        )
    }
}

pub const RAYLIB: Pkg = Pkg {
    dev: "raysan5",
    name: "raylib",
    version: "5.5",
};

pub const RAYGUI: Pkg = Pkg {
    dev: "raysan5",
    name: "raygui",
    version: "4.0",
};

pub const PHYSAC: Pkg = Pkg {
    dev: "victorfisac",
    name: "Physac",
    version: "1.1",
};
