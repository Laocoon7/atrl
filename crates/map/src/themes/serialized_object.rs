#[macro_export]
macro_rules! impl_serialized_object_for {
    ($type:ty) => {
        impl $type {
            pub fn from_file_ron<Path: Into<PathBuf>>(path: Path) -> Result<Self> {
                let contents = read(path)?;
                match ron::de::from_str::<Self>(&contents) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(MyError::Std(Box::new(e))),
                }
            }

            pub fn from_reader_ron<R: std::io::Read>(reader: R) -> Result<Self> {
                match ron::de::from_reader::<R, Self>(reader) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(MyError::Std(Box::new(e))),
                }
            }

            pub fn to_file_ron<Path: Into<PathBuf>>(&self, path: Path) -> Result<()> {
                let config = PrettyConfig::default();
                match ron::ser::to_string_pretty(self, config) {
                    Ok(s) => write(path, &s),
                    Err(e) => Err(MyError::Std(Box::new(e))),
                }
            }

            pub fn to_writer_ron<W: std::io::Write>(&self, writer: W) -> Result<()> {
                let config = PrettyConfig::default();
                match to_writer_pretty(writer, self, config) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(MyError::Std(Box::new(e))),
                }
            }
        }
    };
}
