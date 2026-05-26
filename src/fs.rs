use std::{fs, io};
use std::io::ErrorKind;
use std::path::Path;

pub enum FileOpError {
    WhileReading(io::Error),
    WhileWriting(io::Error),
}

pub fn read_or_default_and_create<P: AsRef<Path> + Clone, V: AsRef<[u8]>>(path: P, default: V) -> Result<Vec<u8>, FileOpError> {
    match fs::read(path.clone()) {
        Ok(data) => Ok(data),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            let default = default;
            match fs::write(path, &default) {
                Ok(v) => Ok(default.as_ref().to_vec()),
                Err(e) => Err(FileOpError::WhileWriting(e)),
            }
        }
        Err(e) => Err(FileOpError::WhileReading(e))
    }
}