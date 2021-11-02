
use tar::Archive;
use std::io::Cursor;
use flate2::read::{GzDecoder};
use tokio::process::Command;
use std::fs::File;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
 
async fn fetch_url(url: String, file_name: String) -> Result<()> {
    // code from https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_name)?;
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


async fn sh(cmd: String, args: Vec<&str>) -> Result<()>{
    let output: std::process::Output = Command::new(cmd)
        .args(args)
        .current_dir("./")
        .output().await?;
    println!("stderr: {:?}", output.stderr);
    Ok(())
}

const JAVA_NAME_COMPRESSED: &str = "java.tar.gz";
const APACHE_TOMCAT_NAME_COMPRESSED: &str = "tomcat.tar.gz";
const SCADA_LTS_NAME: &str = "ScadaBR.war";
const CONTEXT_XML: &str = "context.xml";

const DIR_TOMCAT_UNCONPRESED:&str = "apache-tomcat-9.0.48";
//const DIR_JAVA_UNCONPRESED:&str = "jdk-11.0.13+8";

#[tokio::main]
async fn main() {
    let java_url = String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz");
    let apache_tomcat_url = String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.tar.gz");
    let scada_lts_url = String::from("https://github.com/SCADA-LTS/Scada-LTS/releases/download/v2.6.10-rc1/Scada-LTS.war");
    let default_tomcat_config = String::from("https://github.com/SCADA-LTS/Scada-LTS/blob/develop/docker/config/context.xml");
    
    println!("Start inst Scada-LTS v0.0.1.121");

    //---
    println!("Get java");
    fetch_url(java_url, JAVA_NAME_COMPRESSED.to_string()).await.unwrap();
    println!("Start unpacking");
    uncopresed_tar_gz(JAVA_NAME_COMPRESSED.to_string()).await.unwrap();

    //---
    println!("Get apache-tomcat");
    fetch_url(apache_tomcat_url, APACHE_TOMCAT_NAME_COMPRESSED.to_string()).await.unwrap();
    println!("Start unpacking");
    uncopresed_tar_gz(APACHE_TOMCAT_NAME_COMPRESSED.to_string()).await.unwrap();

    //---
    println!("Get Scada-LTS");
    fetch_url(scada_lts_url, SCADA_LTS_NAME.to_string()).await.unwrap();

    //---
    println!("ScadaBR.war move to tomacat");
    let dir_webapps = format!("./{}/webapps", DIR_TOMCAT_UNCONPRESED);
    let args_sh_move_scada = vec!["-f", "./ScadaBR.war", &dir_webapps];
    sh("mv".to_string(), args_sh_move_scada).await.unwrap();
    

    //---
    println!("Get default tomcat config");
    fetch_url(default_tomcat_config, CONTEXT_XML.to_string()).await.unwrap();
    let dir_cfg = format!("./{}/conf", DIR_TOMCAT_UNCONPRESED);
    let args_sh_move_config_to_tomcat = vec!["-f","./context.xml",&dir_cfg];
    sh("mv".to_string(), args_sh_move_config_to_tomcat).await.unwrap();

    //---
    println!("end");
}

