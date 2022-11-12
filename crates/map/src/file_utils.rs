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
