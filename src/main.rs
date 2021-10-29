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

const JAVA_NAME_COMPRESSED: &str = "java.tar.gz";
const APACHE_TOMCAT_NAME_COMPRESSED: &str = "tomcat.tar.gz";

#[tokio::main]
async fn main() {
    let java_url = String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz");
    let apache_tomcat_url = String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48-deployer.tar.gz");
    
    println!("Start inst Scada-LTS v0.0.1.1");
    //---
    println!("Get java");
    fetch_url(java_url, JAVA_NAME_COMPRESSED.to_string()).await.unwrap();
    println!("unconpresed");
    uncopresed_tar_gz(JAVA_NAME_COMPRESSED.to_string()).await.unwrap();
    //---
    println!("Get apache-tomcat");
    fetch_url(apache_tomcat_url, APACHE_TOMCAT_NAME_COMPRESSED.to_string()).await.unwrap();
    println!("uncompresed");
    uncopresed_tar_gz(APACHE_TOMCAT_NAME_COMPRESSED.to_string()).await.unwrap();
    
    println!("end");
}

