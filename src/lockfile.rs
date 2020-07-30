use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct LockFile {
    file_path: PathBuf,
    lock_path: PathBuf,
    lock: Option<File>,
}

impl LockFile{
    pub fn new(file_path: PathBuf) -> LockFile {
        let lock_path = PathBuf::from(&format!("{}.lock", file_path.to_str().unwrap()));
        LockFile{file_path, lock_path, lock: None}
    }

    pub fn hold_for_update(&mut self) {
        match self.lock {
            Some(_) => panic!("No Permission to open lock file as it is already opened elsewhere"),
            None => {
                self.lock = Some(File::create(&self.lock_path).unwrap());
            }
        }
    }

    pub fn write(&self, msg: String) {
        self.raise_on_stale_lock();
        self.lock.as_ref().unwrap().write(msg.as_ref());
    }

    pub fn commit(&mut self) {
        self.raise_on_stale_lock();
        fs::rename(&self.lock_path, &self.file_path);
        self.lock = None;
    }

    fn raise_on_stale_lock(&self) {
        match self.lock {
            Some(_) => {},
            None => {
                panic!("File lock not created, yet trying to write to file");
            }
        }
    }
}





