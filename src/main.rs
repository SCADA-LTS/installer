//
// (c) 2021 - Installer Scada-LTS
// gbylica@softq.pl, grzegorz.bylica@gmail.com
//

mod inst;

use clap::{App, Arg};
use rpassword::read_password;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process;
use tokio::time::{sleep, Duration};

use const_format::formatcp;

const DIR_TOMCAT_UNCONPRESED: &str = "apache-tomcat-9.0.56";

type Uncopresed = inst::Uncopresed; //m
type Featch = inst::Featch;          //fetch
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
        "Start inst v{} for Scada-LTS v2.6.11",
        &env!("CARGO_PKG_VERSION")
    );

    check_inst();

    let matches = App::new("Installer Scada-LTS")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about("This is to prepare the environment to run Scada-LTS [WARNING] if it already exists in current directory it will overwrite it")
                          .arg(Arg::with_name("mysql_user")
                               .short("u")
                               .long("mysql_user")
                               .help("Setting the mysql user to be used by Scada-LTS \n to connect to the mysql server (default root)")
                               .value_name("mysql_user")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("mysql_password")
                               .short("p")
                               .long("mysql_paswd")
                               .help("Setting the mysql password to be used by Scada-LTS \n to connect to the mysql server (default root)")
                               .value_name("mysql_password")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("mysql_host")
                               .short("h")
                               .long("host")
                               .help("Setting the mysql server host - default 'localhost'")
                               .value_name("mysql_host")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("mysql_port")
                               .short("r")
                               .long("port")
                               .help("Setting the mysql server port - default '3306'")
                               .value_name("mysql_port")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("mysql_database")
                               .short("d")
                               .long("database")
                               .help("Setting the mysql server database - default 'scadalts'")
                               .value_name("mysql_database")
                               .required(false)
                               .takes_value(true))
                          .arg(Arg::with_name("ask_for_mysql_password")
                               .short("a")
                               .long("ask_for_mysql_paswd")
                               .help("Ask for the mysql password to be used by Scada-LTS \n to connect to the mysql server")
                            )
                          .arg(Arg::with_name("default")
                                .short("i")
                                .long("it_is_not_default")
                                .help("It is default value for installation and configuration of all components (including the database) \n with its default values ")
                          )
                          .arg(Arg::with_name("create_db")
                               .short("c")
                               .long("create_db")
                               .help("The name of the database to be created and used to configure Scada-LTS to create the data structures needed to run the program and save the data."))
                               
                          .get_matches();


    let mysql_user = matches.value_of("mysql_user").unwrap_or("root");
    let mysql_password = matches.value_of("mysql_password").unwrap_or("root");
    let mysql_host = matches.value_of("mysql_host").unwrap_or("localhost");
    let mysql_port = matches.value_of("mysql_port").unwrap_or("3306");
    let mysql_db = matches.value_of("mysql_database").unwrap_or("scadalts");

    let ask_for_mysql_password = matches.index_of("ask_for_mysql_password");
    let create_db = matches.index_of("create_db");
    let it_is_not_default = matches.index_of("it_is_not_default");


    //TODO check in this place it was before instalation
    if it_is_not_default.is_none() {
        println!(
            "The default installation will download and unzip \n \
            and run all components needed to run Scada-LTS in the current directory.");

    } else {
        println!("MySql user: {}", mysql_user);
        println!("MySql password: {}", mysql_password);
        println!("Mysql host: {}", mysql_host);
        println!("Mysql port: {}", mysql_port);
        println!("Mysql database: {}", mysql_db);
        println!("MySql ask for mysql password: {:?}", ask_for_mysql_password);
        println!("Mysql createdb: {:?}", create_db);


        if ask_for_mysql_password.is_some() {
            print!("Type a password for MySql user:{} - password: ", mysql_user);
            std::io::stdout().flush().unwrap();
            let mysql_password = read_password().unwrap();
            println!("The password is: '{}'", mysql_password);
        }
    }

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
                url: "https://downloads.mysql.com/archives/get/p/23/file/mysql-5.7.35-win32.zip",
                file_name: "my.zip",
            },
            msg: "Get mysql and unpacking",
        });
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
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
        //TODO for windows;
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

    //---
    let mut to_fetch: Vec<Move> = Vec::new();  

    let dir_webapps = formatcp!("./{}/webapps", DIR_TOMCAT_UNCONPRESED); //m ss
    let dir_lib = formatcp!("./{}/lib", DIR_TOMCAT_UNCONPRESED);

    //https://github.com/SCADA-LTS/Scada-LTS/releases/download/v2.6.10-rc1/Scada-LTS.war
    to_fetch.push(
    Move{
        featch: Featch{
            url: "https://github.com/SCADA-LTS/Scada-LTS/releases/download/v2.6.11/ScadaBR.war",
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

    inst::fetch_and_move(to_fetch).await.unwrap();

    //correct configuration
    //write_user_and_passwd_mysql(mysql_user, mysql_password).await.unwrap();

    //create configuration
    if it_is_not_default.is_none() {
        inst::create_config_xml("root","","localhost","9797","scadalts",
            DIR_TOMCAT_UNCONPRESED,
        ).await.unwrap();
        inst::sh("./my_init.sh").await;
        tokio::task::spawn_blocking(move || inst::sh("./my.sh")).await;
        //inst::sh("./my.sh").await;
        sleep(Duration::from_millis(2000)).await;
        inst::sh("./start.sh").await;
    } else {
        inst::create_config_xml(
            mysql_user,
            mysql_password,
            mysql_host,
            mysql_port,
            mysql_db,
            DIR_TOMCAT_UNCONPRESED,
        ).await.unwrap();
        //TODO create start file
        println!("If you have installed and running mysql server on localhost:3603 with user {} and {} {} and the scadalts database is set up then you can run the \"./start.sh\" program",mysql_user,mysql_password,mysql_user);

    }
    //TODO czy chcesz wystartowac aplikacje
    println!("Start in webrowser - http://localhost:8080/ScadaBR");
    //usuniecie dodakowych aplikacji menager itp
    //wylaczenie portu 8005

    //---
    println!("end");
}
