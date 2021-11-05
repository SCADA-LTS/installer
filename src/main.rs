
use tar::Archive;
use std::io::Cursor;
use flate2::read::{GzDecoder};
use tokio::process::Command;
use std::fs::File;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

const JAVA_NAME_COMPRESSED: &str = "java.tar.gz";
const APACHE_TOMCAT_NAME_COMPRESSED: &str = "tomcat.tar.gz";
const SCADA_LTS_NAME: &str = "ScadaBR.war";
const CONTEXT_XML: &str = "context.xml";

const DIR_TOMCAT_UNCONPRESED:&str = "apache-tomcat-9.0.48";
const MY_SQL_JAR_CONNECTOR:&str = "mysql_connector.jar";
//const DIR_JAVA_UNCONPRESED:&str = "jdk-11.0.13+8";

//Featch 
struct F{
    url: String,
    file_name: String
}
 
async fn fetch_url(url: &str, file_name: &str) -> Result<()> {
    // code from https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn uncopresed_tar_gz(filename: &str) -> Result<()> {
    let tar = File::open(filename)?;
    let dec = GzDecoder::new(tar);
    let mut a = Archive::new(dec);
    a.unpack(".")?;
    Ok(())
}

async fn sh(cmd: &str, args: Vec<&str>) -> Result<()>{
    let output: std::process::Output = Command::new(cmd)
        .args(args)
        .current_dir("./")
        .output().await?;
    println!("stderr: {:?}", output.stderr);
    Ok(())
}

async fn fetch_and_move(to_fetch:Vec<F>) -> Result<()>{
    for f in to_fetch {
        fetch_url(&f.url, &f.file_name).await.unwrap();
        let dir_lib = format!("./{}/lib", DIR_TOMCAT_UNCONPRESED);
        let name = format!("./{}", &f.file_name);
        let args = vec!["-f", &name, &dir_lib];
        let mv = "mv";
        sh(mv, args).await.unwrap();            
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let java_url = String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz");
    let apache_tomcat_url = String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.tar.gz");
    let scada_lts_url = String::from("https://github.com/SCADA-LTS/Scada-LTS/releases/download/v2.6.10-rc1/Scada-LTS.war");
    //let default_tomcat_config = String::from("https://raw.githubusercontent.com/SCADA-LTS/Scada-LTS/develop/docker/config/context.xml");
    let default_tomcat_config = String::from("https://github.com/SCADA-LTS/installer/releases/download/rv0.0.1/context.xml");
    let get_connector_mysql = String::from("https://repo1.maven.org/maven2/mysql/mysql-connector-java/5.1.49/mysql-connector-java-5.1.49.jar");
    
    
    //Embeded sql not now
    //let mysql_5_7_x86 = String::from("https://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.36-linux-glibc2.12-x86_64.tar.gz");
    //https://dev.mysql.com/doc/refman/5.7/en/windows-create-option-file.html

    println!("Start inst v0.0.2 for Scada-LTS v2.6.10");

    //---
    println!("Get java");
    fetch_url(&java_url, &JAVA_NAME_COMPRESSED).await.unwrap();
    println!("Start unpacking");
    uncopresed_tar_gz(&JAVA_NAME_COMPRESSED).await.unwrap();

    //---
    println!("Get apache-tomcat");
    fetch_url(&apache_tomcat_url, &APACHE_TOMCAT_NAME_COMPRESSED).await.unwrap();
    println!("Start unpacking");
    uncopresed_tar_gz(&APACHE_TOMCAT_NAME_COMPRESSED).await.unwrap();

    //---
    println!("Get Scada-LTS");
    fetch_url(&scada_lts_url, &SCADA_LTS_NAME).await.unwrap();

    //---
    println!("ScadaBR.war move to tomacat");
    let dir_webapps = format!("./{}/webapps", &DIR_TOMCAT_UNCONPRESED);
    let args_sh_move_scada = vec!["-f", "./ScadaBR.war", &dir_webapps];
    sh("mv", args_sh_move_scada).await.unwrap();
    
    //---
    println!("Get default tomcat config");
    fetch_url(&default_tomcat_config, &CONTEXT_XML).await.unwrap();
    let dir_cfg = format!("./{}/conf", &DIR_TOMCAT_UNCONPRESED);
    let args_sh_move_config_to_tomcat = vec!["-f","./context.xml",&dir_cfg];
    sh("mv", args_sh_move_config_to_tomcat).await.unwrap();


    //---
    println!("Get library to connect mysql");
    fetch_url(&get_connector_mysql, &MY_SQL_JAR_CONNECTOR).await.unwrap();
    let dir_lib = format!("./{}/lib", DIR_TOMCAT_UNCONPRESED);
    let args_sh_move_con_mysql_to_tomcat = vec!["-f","./mysql_connector.jar",&dir_lib];
    sh("mv", args_sh_move_con_mysql_to_tomcat).await.unwrap();

    
    //---
    println!("Get library for extends tomcat");
    let mut to_fetch: Vec<F> = Vec::new();
    to_fetch.push(
        F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/activation.jar"),
            //url: String::from("https://github.com/SCADA-LTS/Scada-LTS/blob/develop/tomcat/lib/activation.jar"),
            file_name: String::from("activation.jar"),
        });
    to_fetch.push(    
        F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-api-2.4.0-b180830.0359.jar"),
            //url:String::from("https://github.com/SCADA-LTS/Scada-LTS/blob/develop/tomcat/lib/jaxb-api-2.4.0-b180830.0359.jar"),
            file_name: String::from("jaxb-api-2.4.0-b180830.0359.jar"),
        });
    to_fetch.push(
        F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-core-3.0.2.jar"),
            //url:String::from("https://github.com/SCADA-LTS/Scada-LTS/blob/develop/tomcat/lib/jaxb-core-3.0.2.jar"),
            file_name: String::from("jaxb-core-3.0.2.jar"),
        });
    to_fetch.push(
        F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar"),
            //url:String::from("https://github.com/SCADA-LTS/Scada-LTS/blob/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar"),
            file_name: String::from("jaxb-runtime-2.4.0-b180830.0438.jar"),
        });

    fetch_and_move(to_fetch).await.unwrap();

    //usuniecie dodakowych aplikacji menager itp
    //wylaczenie portu 8005
    println!("If you have installed and running mysql server on localhost:3603 with user root and password root and the scadalts database is set up then you can run the \"./start.sh\" program");

    //---
    println!("end");
}

