pub use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::prelude::*;

pub fn read<Path: Into<PathBuf>>(path: Path) -> Result<String> {
    let path: PathBuf = path.into();
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(MyError::Io(e)),
    }
}

pub fn write<Path: Into<PathBuf>>(path: Path, value: &str) -> Result<()> {
    let path: PathBuf = path.into();
    let path_string = match path.to_str() {
        Some(s) => s.to_string(),
        None => return Err(MyError::NotAString),
    };

    match path.try_exists() {
        Ok(b) => {
            if !b {
                match path.parent() {
                    Some(dir) => {
                        #[cfg(feature = "debug")]
                        debug!("Creating directory: {:?}", dir);
                        if let Err(e) = std::fs::create_dir_all(dir) {
                            return Err(MyError::Io(e));
                        }
                    }
                    None => return Err(MyError::NotADir(path_string)),
                };
            }
            #[cfg(feature = "debug")]
            debug!("Writing to file: {:?}", path);
            match std::fs::write(path, value) {
                Ok(_) => Ok(()),
                Err(e) => Err(MyError::Io(e)),
            }
        }
        Err(e) => Err(MyError::Io(e)),
    }
}

pub fn get_files_with_extension(path: &str, extension: &str) -> Result<Vec<String>> {
    get_files_with_extensions(path, vec![extension])
}

pub fn get_files_with_extensions(path: &str, extensions: Vec<&str>) -> Result<Vec<String>> {
    let mut ret = Vec::new();
    println!("{}", path);
    let paths = std::fs::read_dir(path)?;
    for dir_entry in paths {
        if let Ok(dir) = dir_entry {
            let path = dir.path();
            println!("{:?}", path);
            if let Some(ext) = path.extension() {
                let mut found = false;
                for extension in &extensions {
                    if ext == *extension {
                        found = true;
                        break;
                    }
                }
                if found {
                    if let Some(file_name) = path.file_name() {
                        if let Some(path_name) = file_name.to_str() {
                            ret.push(path_name.to_string());
                        }
                    }
                }
            }
        }
    }
    Ok(ret)
}

pub fn strip_extension(path: &str) -> Option<String> {
    let path = Path::new(path);
    if let Some(p) = path.with_extension("").to_str() {
        Some(p.to_string())
    } else {
        None
    }
}
