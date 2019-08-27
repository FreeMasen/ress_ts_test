use ress::prelude::*;
use std::error::Error;
use reqwest;
use flate2::read::GzDecoder;

use walkdir::WalkDir;

static TS_URL: &str = "https://github.com/microsoft/TypeScript/archive/master.tar.gz";

fn main() -> Result<(), Box<dyn Error>> {
    let buf = ::std::path::PathBuf::from("typescript/TypeScript-master/tests/projects");
    if !buf.exists() {
        download_ts_files()?;
    }
    for entry in WalkDir::new("typescript/TypeScript-master/tests/projects") {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let file_type = entry.file_type();
        if file_type.is_dir() {
            continue;
        } else if file_type.is_file() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "ts" {
                test_file(path)?;
                }
            }
        }
    }
    Ok(())
}

fn test_file<T: AsRef<::std::path::Path>>(path: T) -> Result<(), Box<dyn Error>> {
    let ts = ::std::fs::read_to_string(path.as_ref())?;
    println!("================");
    println!("starting {:?}", path.as_ref());
    println!("================");
    for token in Scanner::new(&ts) {
        println!("{}", format(token.unwrap()));
    }
    Ok(())
}

fn format(item: Item<Token<&str>>) -> String {
    format!("{:?}::{}:{}", item.token, item.span.start, item.span.end)
}

fn download_ts_files() -> Result<(), Box<dyn Error>> {
    let mut tar_gz = reqwest::get(TS_URL)?;
    let mut buf = Vec::new();
    tar_gz
        .copy_to(&mut buf)
        .expect("failed to copy to BzDecoder");
    let gz = GzDecoder::new(buf.as_slice());
    let mut t = tar::Archive::new(gz);
    t.unpack("typescript").expect("Failed to unpack gz");
    Ok(())
}