use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::BufReader;
use std::io::Result as IoResult;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct SystemEntry {
    pub unique_filename: String,
    pub filename: String,
    pub hash: String,
    pub hash256: String,
    pub length: usize,
    pub content_type: String,
    pub uploaded: String,
    pub storage_type: String,
    pub do_not_cache: bool,
}

impl SystemEntry {
    pub fn new(filename: String, data: Vec<u8>) -> Self {
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        sha1.update(&data);
        sha256.update(&data);
        let sha1 = sha1.finalize();
        let sha256 = sha256.finalize();
        
        SystemEntry {
            unique_filename: filename.clone(),
            filename: filename,
            hash: format!("{:x}", sha1),
            hash256: format!("{:x}", sha256),
            length: data.len(),
            content_type: String::from("application/octet-stream"),
            uploaded: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            storage_type: String::from("S3"),
            do_not_cache: true,
        }
    }
    
    pub fn from_file<P: AsRef<Path>>(path: P) -> IoResult<Self> {
        let path = path.as_ref();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        
        let mut buf_reader = BufReader::new(File::open(path)?);
        let mut data = Vec::new();
        
        buf_reader.read_to_end(&mut data)?;
        
        Ok(Self::new(name, data))
    }
}
