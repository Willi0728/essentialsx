use std::{fs, io};
use std::io::ErrorKind;
use std::path::Path;

pub enum FileOpError {
    WhileReading(io::Error),
    WhileWriting(io::Error),
}

pub fn read_or_default_and_create<P: AsRef<Path>, V: AsRef<[u8]>>(path: P, default: V) -> Result<Vec<u8>, FileOpError> {
    match fs::read(&path) {
        Ok(data) => Ok(data),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            match fs::write(path, &default) {
                Ok(()) => Ok(default.as_ref().to_vec()),
                Err(e) => Err(FileOpError::WhileWriting(e)),
            }
        }
        Err(e) => Err(FileOpError::WhileReading(e))
    }
}