use std::{fs::File, io::Read};

use cloud_storage::{Bucket, Client};
use log::debug;

pub async fn connect_bucket(bucket_name: &str) -> Option<Bucket> {
    let client = Client::default();
    match client.bucket().read(bucket_name).await {
        Ok(bucket) => {
            debug!("successfully accessed bucket {bucket_name}");
            Some(bucket)
        }
        Err(e) => {
            eprintln!("bucket read failed: {}", e);
            None
        }
    }
}

pub async fn write_to_bucket(filename: &str, bucket_name: &str) {
    let mut bytes: Vec<u8> = Vec::new();
    match File::open(filename) {
        Ok(file) => {
            debug!("successfully opened file {filename}");
            for byte in file.bytes() {
                match byte {
                    Ok(byte) => {
                        bytes.push(byte);
                    }
                    Err(e) => {
                        debug!("error while rading byte: {e}");
                        return;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("error openning the file {filename}: {e}");
            return;
        }
    };
    let client = Client::default();
    match client
        .object()
        .create(bucket_name, bytes, filename, "text/plain")
        .await
    {
        Ok(_) => debug!("successfully uploaded file {filename}"),
        Err(e) => eprintln!("error during file {filename} upload : {e}"),
    };
}

pub async fn read_from_bucket(filename: &str, bucket_name: &str) {
    let bucket = match Client::default().bucket().read(bucket_name).await {
        Ok(_) => debug!("successfully read bucket {bucket_name}"),
        Err(e) => eprintln!("error during read of bucket: {e}"),
    };
}
