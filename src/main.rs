use std::fs::File;
use tar::Archive;
use std::io::Cursor;
use flate2::read::{GzDecoder};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    // code from https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn uncopresed_tar_gz(filename: String) -> Result<()> {
    let tar = File::open(filename)?;
    let dec = GzDecoder::new(tar);
    let mut a = Archive::new(dec);
    a.unpack(".")?;
    Ok(())
}

const JAVA_NAME: &str = "java.tar.gz";

#[tokio::main]
async fn main() {
    let java_url = String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz");
    
    println!("Start inst Scada-LTS v0.0.1.1");
    println!("Get java");
    fetch_url(java_url, JAVA_NAME.to_string()).await.unwrap();
    println!("unconpresed");
    uncopresed_tar_gz(JAVA_NAME.to_string()).await.unwrap();
    
    println!("end");
}

