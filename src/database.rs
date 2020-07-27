extern crate crypto;
use crypto::sha1::Sha1;
use crypto::digest::Digest;

extern crate rand;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

extern crate deflate;
use deflate::write::ZlibEncoder;
use deflate::Compression;

use crate::blob::Blob;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
pub struct Database<'a> {
    path: &'a Path,
}

impl Database<'_> {
    pub fn new(path: &Path) -> Database {
        Database{path}
    }

    pub fn store(self, object: Blob, type_val: String) -> String {
        let string = object.to_string();
        let content = format!("{} {}\0{}", type_val, string.len(), string);
        let mut hasher = Sha1::new();
        hasher.input_str(&content);
        let id = &hasher.result_str();
        self.write_object(&id, &content);
        id.to_string()
    }

    fn write_object(self, oid: &String, content: &String) {
        let dir_path = self.path.join(&oid[0..2]);
        if !dir_path.exists() {
            fs::create_dir(&dir_path);
        }
        let tmp_path = &dir_path.join(Database::generate_temp_name());
        let object_path = &dir_path.join(&oid[2..(oid.len())]);

        let mut file = File::create(tmp_path).unwrap();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Default);
        encoder.write_all(content.as_bytes()).expect("Write error!");
        let compressed = encoder.finish().expect("Failed to finish compression");
        file.write(compressed.as_ref());

        fs::rename(tmp_path, object_path);
    }

    fn generate_temp_name() -> String {
        let rand_chars: String = thread_rng().sample_iter(&Alphanumeric).take(6).collect();
        format!("tmp_obj_{}", rand_chars)
    }
        
}
