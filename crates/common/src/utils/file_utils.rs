pub use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::prelude::*;
pub fn read_str<Path: Into<PathBuf>>(path: Path) -> AtrlResult<String> {
    let path: PathBuf = path.into();
    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(AtrlError::Io(e)),
    }
}
pub fn write_str<Path: Into<PathBuf>>(path: Path, value: &str) -> AtrlResult<()> {
    let path: PathBuf = path.into();
    let path_string = match path.to_str() {
        Some(s) => s.to_string(),
        None => return Err(AtrlError::NotAString),
    };
    match path.try_exists() {
        Ok(b) => {
            if !b {
                match path.parent() {
                    Some(dir) => {
                        #[cfg(feature = "debug")]
                        debug!("Creating directory: {:?}", dir);
                        if let Err(e) = std::fs::create_dir_all(dir) {
                            return Err(AtrlError::Io(e));
                        }
                    },
                    None => return Err(AtrlError::NotADir(path_string)),
                };
            }
            #[cfg(feature = "debug")]
            debug!("Writing to file: {:?}", path);
            match std::fs::write(path, value) {
                Ok(_) => Ok(()),
                Err(e) => Err(AtrlError::Io(e)),
            }
        },
        Err(e) => Err(AtrlError::Io(e)),
    }
}
pub fn get_files_with_extension<Path: Into<PathBuf>>(
    path: Path,
    extension: &str,
) -> AtrlResult<Vec<String>> {
    get_files_with_extensions(path, vec![extension])
}
pub fn get_files_with_extensions<Path: Into<PathBuf>>(
    path: Path,
    extensions: Vec<&str>,
) -> AtrlResult<Vec<String>> {
    let mut ret = Vec::new();
    let path: PathBuf = path.into();
    let paths = std::fs::read_dir(&path)?;
    //#[cfg(feature = "debug")]
    {
        let mut extension_names = String::new();
        let mut first = true;
        for extension in &extensions {
            if first {
                extension_names = extension.to_string();
                first = false;
            } else {
                extension_names = format!("{}, {}", extension_names, extension);
            }
        }
        info!("Looking for {{{}}} files in {:?}", extension_names, path);
    }
    for dir_entry in paths.flatten() {
        let path = dir_entry.path();
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
    //#[cfg(feature = "debug")]
    {
        let mut found_names = String::new();
        let mut first = true;
        for file in &ret {
            if first {
                found_names = format!("\t{}", file);
                first = false;
            } else {
                found_names = format!("{}\n\t{}", found_names, file);
            }
        }
        info!("Found {{\n{}\n}}", found_names);
    }
    Ok(ret)
}
pub fn strip_extension(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.with_extension("").to_str().map(|p| p.to_string())
}
