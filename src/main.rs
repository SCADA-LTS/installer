//
// (c) 2021 - Installer Scada-LTS
// gbylica@softq.pl, grzegorz.bylica@gmail.com
//

mod inst;

use clap::{App};
use std::env;
use std::path::Path;
use std::process::{self, Command};

use const_format::formatcp;

const DIR_TOMCAT_UNCONPRESED: &str = "apache-tomcat-9.0.56";

type Uncopresed = inst::Uncopresed; 
type Featch = inst::Featch;          
type Move = inst::Move;

fn check_inst() {

    let tomcat = Path::new(DIR_TOMCAT_UNCONPRESED).exists();
    let jdk = Path::new("jdk-11.0.13+8").exists();
    let mysql = Path::new("mysql-5.7.36-linux-glibc2.12-x86_64").exists();
    let my_shel = Path::new("mysql-shell-8.0.27-linux-glibc2.12-x86-64bit").exists();

    if tomcat | jdk | mysql | my_shel {
        println!("It is an installation or part of it in the current directory, please install it in another directory.");
        process::exit(1);
    }
}

#[tokio::main]
async fn main() {

    println!(
        "Start inst v{} for Scada-LTS v2.6.13",
        &env!("CARGO_PKG_VERSION")
    );

    check_inst();

    App::new("Installer Scada-LTS")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about("This is to prepare the environment to run Scada-LTS [WARNING] if it already exists in current directory it will overwrite it");

    println!("Internal Mysql on port: 9797");
    println!("Internal tomcat on port: 8080");
    println!("Internal java 1.11");

    let mut to_fetch_and_unpacking: Vec<Uncopresed> = Vec::new();

    if cfg!(target_os = "windows") {
    to_fetch_and_unpacking.push(
    Uncopresed{
        featch: Featch{ url: "https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x86-32_windows_hotspot_11.0.13_8.zip",
                        file_name: "java.zip"
                      },
        msg: "Get java and unpacking"
    });
    to_fetch_and_unpacking.push(
    Uncopresed{
        featch: Featch{ url: "https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.56/bin/apache-tomcat-9.0.56.zip",
                        file_name: "tomcat.zip",
                      },
        msg: "Get tomcat and unpacking"
    });
    to_fetch_and_unpacking.push(Uncopresed {
        featch: Featch {
            url: "https://downloads.mysql.com/archives/get/p/23/file/mysql-5.7.35-win64.zip",
            file_name: "my.zip",
        },
        msg: "Get mysql and unpacking",
    });
 
    to_fetch_and_unpacking.push(
            Uncopresed{
                featch: Featch{
                      url: "https://downloads.mysql.com/archives/get/p/43/file/mysql-shell-1.0.11-windows-x86-64bit.zip",
                      file_name: "myshell.zip",
                },
                msg: "Get mysql shell and unpacking"
            });

    } else {

    
    to_fetch_and_unpacking.push(
        Uncopresed{
            featch: Featch{ url: "https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz",
                            file_name: "java.tar.gz"
                          },
            msg: "Get java and unpacking"
        });
    to_fetch_and_unpacking.push(
        Uncopresed{
            featch: Featch{
                  url: "https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.56/bin/apache-tomcat-9.0.56.tar.gz",
                  file_name: "tomcat.tar.gz",
                },
            msg: "Get tomcat and unpacking"
        });
    to_fetch_and_unpacking.push(
            Uncopresed{
                featch: Featch{
                      url: "https://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.36-linux-glibc2.12-x86_64.tar.gz",
                      file_name: "my.tar.gz",
                    },
                msg: "Get mysql and unpacking"
            });
    
    to_fetch_and_unpacking.push(
            Uncopresed{
                featch: Featch{
                      url: "https://dev.mysql.com/get/Downloads/MySQL-Shell/mysql-shell-8.0.27-linux-glibc2.12-x86-64bit.tar.gz",
                      file_name: "myshell.tar.gz",
                },
                msg: "Get mysql shell and unpacking"
            });
    }

    inst::fetch_and_uncopresed(to_fetch_and_unpacking)
      .await
      .unwrap();

    let mut to_fetch: Vec<Move> = Vec::new();  

    let dir_webapps = formatcp!("./{}/webapps", DIR_TOMCAT_UNCONPRESED); 
    let dir_lib = formatcp!("./{}/lib", DIR_TOMCAT_UNCONPRESED);

    //2.6.13
    to_fetch.push(
    Move{
        featch: Featch{
            url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/ScadaBR.war",
            file_name: "ScadaBR.war"
        },
        msg: "Get Scada-LTS - and move to tomcat as ScadaBR.war",
        to_dir: dir_webapps
    });
    to_fetch.push(
    Move{
        featch: Featch{
            url: "https://repo1.maven.org/maven2/mysql/mysql-connector-java/5.1.49/mysql-connector-java-5.1.49.jar",
            file_name: "mysql_connector.jar"
        },
        msg: "Get lib - mysql-connector-java-5.1.49.jar - and move to tomcat",
        to_dir: dir_lib
    });
    to_fetch.push(Move {
        featch: Featch {
            url: "https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/activation.jar",
            file_name: "activation.jar",
        },
        msg: "Get lib - activation.jar and move to tomcat",
        to_dir: dir_lib,
    });
    to_fetch.push(Move{
        featch: Featch{ url: "https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-api-2.4.0-b180830.0359.jar",
                        file_name: "jaxb-api-2.4.0-b180830.0359.jar"
                       },
        msg: "Get lib - jaxb-api-2.4.0-b180830.0359.jar and move to tomcat",
        to_dir: dir_lib
    });
    to_fetch.push(Move {
        featch: Featch {
            url:
                "https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-core-3.0.2.jar",
            file_name: "jaxb-core-3.0.2.jar",
        },
        msg: "Get lib - jaxb-core-3.0.2.jar and move to tomcat",
        to_dir: dir_lib,
    });
    to_fetch.push(
    Move{
        featch: Featch {
            url: "https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar",
            file_name: "jaxb-runtime-2.4.0-b180830.0438.jar"
        },
        msg: "Get lib - jaxb-runtime-2.4.0-b180830.0438.jar and move to tomcat",
        to_dir: dir_lib
    });

    if cfg!(target_os = "windows") {

        to_fetch.push(
        Move{
            featch: Featch {
                url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/my_init.bat",
                file_name: "my_init.sh"
            },
            msg: "Get script - my_init.bat",
            to_dir: "./"
        });

    to_fetch.push(
        Move{
            featch: Featch {
                url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/my.bat",
                file_name: "my.sh"
            },
            msg: "Get script - my.bat",
            to_dir: "./"
        });

    to_fetch.push(
        Move{
            featch: Featch {
                url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/cdb.bat",
                file_name: "cdb.sh"
            },
            msg: "Get script - cdb.bat",
            to_dir: "./"
        });

    to_fetch.push(
        Move{
            featch: Featch {
                url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/start.bat",
                file_name: "start.bat"
            },
            msg: "Get script - start.bat",
            to_dir: "./"
        });




        inst::fetch_and_move(to_fetch).await.unwrap();
        inst::create_config_xml("root","","localhost","9797","scadalts",
            DIR_TOMCAT_UNCONPRESED,
        ).await.unwrap();

        
    

        // let output = Command::new("sh")
        //     .arg("-c")
        //     .arg("chmod +x ./start.sh")
        //     .output()
        //     .expect("failed to execute process");
        //     let startsh = output.stdout;
    
        //    println!("{:?}",startsh.to_owned());


            print!(" Run on command line in the current directory ./start.bat ");
            println!("Then start in webrowser - http://localhost:8080/ScadaBR");
    } else {
    to_fetch.push(
            Move{
                featch: Featch {
                    url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/my_init.sh",
                    file_name: "my_init.sh"
                },
                msg: "Get script - my_init.sh",
                to_dir: "./"
            });

        to_fetch.push(
            Move{
                featch: Featch {
                    url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/my.sh",
                    file_name: "my.sh"
                },
                msg: "Get script - my.sh",
                to_dir: "./"
            });

        to_fetch.push(
            Move{
                featch: Featch {
                    url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/cdb.sh",
                    file_name: "cdb.sh"
                },
                msg: "Get script - cdb.sh",
                to_dir: "./"
            });

        to_fetch.push(
            Move{
                featch: Featch {
                    url: "https://github.com/SCADA-LTS/installer/releases/download/resource_2613/start.sh",
                    file_name: "start.sh"
                },
                msg: "Get script - start.sh",
                to_dir: "./"
            });
        

        inst::fetch_and_move(to_fetch).await.unwrap();
        inst::create_config_xml("root","","localhost","9797","scadalts",
            DIR_TOMCAT_UNCONPRESED,
        ).await.unwrap();
    

        let output = Command::new("sh")
            .arg("-c")
            .arg("chmod +x ./start.sh")
            .output()
            .expect("failed to execute process");
            let startsh = output.stdout;
    
            println!("{:?}",startsh.to_owned());


            print!(" Run on command line in the current directory ./start.sh ");
            println!("Then start in webrowser - http://localhost:8080/ScadaBR");
    }
}