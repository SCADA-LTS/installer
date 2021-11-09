
use tar::Archive;
use std::io::Cursor;
use flate2::read::{GzDecoder};
use tokio::process::Command;
use std::fs::File;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const DIR_TOMCAT_UNCONPRESED:&str = "apache-tomcat-9.0.48";

//Featch
struct F{
    url: String,
    file_name: String
}

//Featch and Uncopresed
struct U{
    f: F,
    msg: String
}

//Featch and Move
struct M{
    f: F,
    msg: String,
    to_dir: String
}
 
async fn fetch_url(url: &str, file_name: &str) -> Result<()> {
    // code from https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_name)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn uncopresed(filename: &str) -> Result<()> {
    let file = File::open(filename)?;
    let dec = GzDecoder::new(file);
    let mut a = Archive::new(dec);
    a.unpack(".")?;
    Ok(())
}

async fn cmd(cmd: &str, args: Vec<&str>, dir: &str) -> Result<()>{
    let output: std::process::Output = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output().await?;
    println!("stderr: {:?}", output.stderr);
    Ok(())
}

async fn _move(file_name: &str, to_dir: &str) {
    if cfg!(target_os = "windows") {
        cmd("move", vec![&file_name, &to_dir],"./").await.unwrap();
    } else if cfg!(target_os = "linux") {
        let args_sh_move = vec![ "-f", &file_name, &to_dir];
        cmd("mv", args_sh_move,"./").await.unwrap();
    }
}

async fn fetch_and_move(to_fetch:Vec<M>) -> Result<()>{
    for m in to_fetch {
        println!("{}",&m.msg);
        fetch_url(&m.f.url, &m.f.file_name).await.unwrap();
        _move(&m.f.file_name, &m.to_dir).await;
    }
    Ok(())
}

async fn fetch_and_uncopresed(to_fetch:Vec<U>) -> Result<()>{
    for u in to_fetch {
        println!("{}", &u.msg);
        fetch_url(&u.f.url, &u.f.file_name).await.unwrap();
        uncopresed(&u.f.file_name).await.unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn main() {
        
    //Embeded sql not now
    //let mysql_5_7_x86 = String::from("https://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.36-linux-glibc2.12-x86_64.tar.gz");
    //https://dev.mysql.com/doc/refman/5.7/en/windows-create-option-file.html

    println!("Start inst v0.3.0 for Scada-LTS v2.6.10");

    //---
    let mut to_fetch_and_unpacking: Vec<U> = Vec::new();
    if cfg!(target_os = "windows") {
      to_fetch_and_unpacking.push(
        U{
            f: F{ 
                  url: String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x86-32_windows_hotspot_11.0.13_8.zip"),
                  file_name: String::from("java.tar.gz")
               },
            msg: String::from("Get java and unpacking")
        });
        to_fetch_and_unpacking.push(
        U{
            f: F{
                  url: String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.zip"),
                  file_name: String::from("tomcat.tar.gz"),
                },
            msg: String::from("Get tomcat and unpacking")
        });
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        to_fetch_and_unpacking.push(
            U{
                f: F{ 
                      url: String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz"),
                      file_name: String::from("java.tar.gz")
                   },
                msg: String::from("Get java and unpacking")
            });
            to_fetch_and_unpacking.push(
            U{
                f: F{
                      url: String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.tar.gz"),
                      file_name: String::from("tomcat.tar.gz"),
                    },
                msg: String::from("Get tomcat and unpacking")
            });
    }
    fetch_and_uncopresed(to_fetch_and_unpacking).await.unwrap();

        
    //---
    let mut to_fetch: Vec<M> = Vec::new();
    
    let dir_webapps = format!("./{}/webapps", &DIR_TOMCAT_UNCONPRESED);
    let dir_cfg = format!("./{}/conf", &DIR_TOMCAT_UNCONPRESED);

    to_fetch.push(
    M{
        f: F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/releases/download/v2.6.10-rc1/Scada-LTS.war"),
            file_name: String::from("ScadaBR.war")
        },
        msg: String::from("Get Scada-LTS - and move to tomcat as ScadaBR.war"),
        to_dir: dir_webapps
    });
    to_fetch.push(
    M{
        f: F{
            url: String::from("https://github.com/SCADA-LTS/installer/releases/download/rv0.0.1/context.xml"),
            file_name: String::from("context.xml")
        },
        msg: String::from("Get config context.xml - and move to tomcat"),
        to_dir: dir_cfg
    });    
    to_fetch.push(
    M{
        f: F{
            url: String::from("https://repo1.maven.org/maven2/mysql/mysql-connector-java/5.1.49/mysql-connector-java-5.1.49.jar"),
            file_name: String::from("mysql_connector.jar")
        },
        msg: String::from("Get lib - mysql-connector-java-5.1.49.jar - and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });    
    to_fetch.push(
    M{
        f: F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/activation.jar"),
            file_name: String::from("activation.jar")
        },
        msg: String::from("Get lib - activation.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(    
    M{
        f: F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-api-2.4.0-b180830.0359.jar"),
            file_name: String::from("jaxb-api-2.4.0-b180830.0359.jar")
        },
        msg: String::from("Get lib - jaxb-api-2.4.0-b180830.0359.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(
    M{
        f: F{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-core-3.0.2.jar"),
            file_name: String::from("jaxb-core-3.0.2.jar")
        },
        msg: String::from("Get lib - jaxb-core-3.0.2.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(
    M{
        f: F {
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar"),
            file_name: String::from("jaxb-runtime-2.4.0-b180830.0438.jar")
        },
        msg: String::from("Get lib - jaxb-runtime-2.4.0-b180830.0438.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });

    fetch_and_move(to_fetch).await.unwrap();
    

    //usuniecie dodakowych aplikacji menager itp
    //wylaczenie portu 8005
    println!("If you have installed and running mysql server on localhost:3603 with user root and password root and the scadalts database is set up then you can run the \"./start.sh\" program");

    //---
    println!("end");
}

