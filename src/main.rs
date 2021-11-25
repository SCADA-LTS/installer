//
// (c) 2021 - Instalator Scada-LTS
// gbylica@softq.pl, grzegorz.bylica@gmail.com
//

use clap::{App, Arg};
use flate2::read::{GzDecoder};
use rpassword::read_password;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use std::path::Path;
use tar::Archive;
use tokio::process::Command;
use std::ffi::OsStr;
use std::io;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const DIR_TOMCAT_UNCONPRESED: &str = "apache-tomcat-9.0.48";

///Featch and Uncopresed - The structure needed to take a file from the `url` resource and save it as `file_name` and unpack it in the current directory, and display the `msg` information
struct Uncopresed {
    ///Structure needed to retrieve a file from resource `featch`
    featch: Featch,
    ///Display the `msg` information
    msg: String,
}

///Featch - Structure needed to retrieve a file from resource `url` and save as `file_name`
struct Featch {
    ///Retrive a file from resource `url`
    url: String,
    ///Save as `file_name`
    file_name: String,
}

/// Decoding type
enum Decoder {
    Zip,
    Gz,
}

///Featch and Move - Structure needed to take a file from the resource `url` and save it as `file_name` and move it to the specified directory - `to_dir`, and display the `msg` information
struct Move {
    ///Structure needed to retrieve a file from resource `featch`
    featch: Featch,
    ///Display the `msg` information
    msg: String,
    ///Move it to the specified directory `to_dir`
    to_dir: String,
}

/// Featch file from `url` and save as `file_name`
///
/// # Arguments
///
/// * `url` - A String - resource URL from where the file will be downloaded
/// * `file_name` - A String - the downloaded file will be saved as the specified filename
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// fetch_url("https://github.com/SCADA-LTS/installer/releases/download/v0.0.2/start.sh", "down_start.sh");
/// ```
async fn fetch_url(url: &str, file_name: &str) -> Result<()> {
    // code from https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
    let response = reqwest::get(url).await?;
    let mut file = File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

async fn unzip(filename: &str) -> Result<()> {
    
    let fname = std::path::Path::new(filename);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}

/// Extracting the file - `file_name`
///
/// # Arguments
///
/// * `file_name` - A String - filename to be extracted
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// uncopresed("tomcat.tar.gz");
/// ```
async fn uncopresed(filename: &str, decoder: Decoder) -> Result<()> {
    let file = File::open(filename)?;
    
    match decoder {
        Decoder::Gz => { 
            let dec = GzDecoder::new(file);
            let mut a = Archive::new(dec);
            a.unpack(".")?;
        },
        Decoder::Zip => { 
            unzip(filename).await.unwrap();
        },
    }
    Ok(())
}

/// Execution of a command `cmd` with arguments `args` in the directory `dir`
///
/// # Arguments
///
/// * `cmd` - A String - command 
/// * `args` - Vec<&str> - args
/// * `dir` - &str - current dir
///
/// # Examples
///
///  ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// cmd("mv", vec!["-f", &file_name, &to_dir], "./");
/// ```
async fn cmd(cmd: &str, args: Vec<&str>, dir: &str) -> Result<()> {
    let output: std::process::Output = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .output()
        .await?;
    println!("stderr: {:?}", output.stderr);
    Ok(())
}

/// Move file to another directory
///
/// # Arguments
///
/// * `filename` - A String - name of file to be moved
/// * `to_dir` - &str - target directory
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// move_file("lib.o", "/");
/// ```
async fn move_file(file_name: &str, to_dir: &str) {
    if cfg!(target_os = "windows") {
        println!("move file-name: {}, to_dir: {}", &file_name, &to_dir);
        cmd("move", vec![&file_name, &to_dir], "./").await.unwrap();
    } else if cfg!(target_os = "linux") {
        let args_sh_move = vec!["-f", &file_name, &to_dir];
        cmd("mv", args_sh_move, "./").await.unwrap();
    }
}

/// Download and move
///
/// # Arguments
///
/// * `to_fetch` - Vec<Move> - vec of struct `Move`
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// let mut to_fetch: Vec<Move> = Vec::new();
/// to_fetch =.push(
///   Move{
///    featch: Featch {
///        url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar"),
///        file_name: String::from("jaxb-runtime-2.4.0-b180830.0438.jar")
///    },
///    msg: String::from("Get lib - jaxb-runtime-2.4.0-b180830.0438.jar and move to tomcat"),
///    to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
/// });
/// 
/// fetch_and_move(to_fetch);
/// ```
async fn fetch_and_move(to_fetch: Vec<Move>) -> Result<()> {
    for m in to_fetch {
        println!("{}", &m.msg);
        fetch_url(&m.featch.url, &m.featch.file_name).await.unwrap();
        move_file(&m.featch.file_name, &m.to_dir).await;
    }
    Ok(())
}

/// Download and uncopresed
///
/// # Arguments
///
/// * `to_fetch` - Vec<Uncopresed> - vec of struct `Uncopresed`
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// let mut to_fetch: Vec<Uncopresed> = Vec::new();
/// to_fetch =.push(
///   Move{
///    featch: Featch {
///        url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar"),
///        file_name: String::from("jaxb-runtime-2.4.0-b180830.0438.jar")
///    },
///    msg: String::from("Get lib - jaxb-runtime-2.4.0-b180830.0438.jar and move to tomcat"),
///    to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
/// });
/// 
/// fetch_and_move(to_fetch);
/// ```
async fn fetch_and_uncopresed(to_fetch: Vec<Uncopresed>) -> Result<()> {
    for u in to_fetch {
        println!("{}", &u.msg);
        fetch_url(&u.featch.url, &u.featch.file_name).await.unwrap();
        let exten = Path::new(&u.featch.file_name).extension().and_then(OsStr::to_str).unwrap();
        
        if exten == "zip" {
            println!("uncomprese: zip extend: {}", exten);
            uncopresed(&u.featch.file_name, Decoder::Zip).await.unwrap();
        } else {
            println!("uncomprese: gz extend: {}", exten);
            uncopresed(&u.featch.file_name, Decoder::Gz).await.unwrap();
        }
    }
    Ok(())
}


/// create configuration to connect database
///
/// # Arguments
///
/// * `mysql_user` - &str- mysql user
/// * `mysql_passwd` - &str - mysql password
/// * `host` - &str - host server mysql
/// * `port` - &str - port server mysql
/// * `database` &str - database for scadalts
/// 
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
/// 
/// create_config_xml("root", "root", "localhost", "3306", "scadalts");
/// ```
async fn create_config_xml(mysql_user: &str, mysql_passwd: &str, host: &str, port: &str, db: &str) -> Result<()> {
    let out_dir = format!("./{}/conf", DIR_TOMCAT_UNCONPRESED);
    let dest_path = Path::new(&out_dir).join("context.xml");

    let context = format!(
        "
        <Context>
        <WatchedResource>WEB-INF/web.xml</WatchedResource>
        <Resource 
            name=\"jdbc/scadalts\" 
            auth=\"Container\" 
            type=\"javax.sql.DataSource\" 
            factory=\"org.apache.tomcat.jdbc.pool.DataSourceFactory\" 
            testWhileIdle=\"true\" 
            testOnBorrow=\"true\" 
            testOnReturn=\"false\" 
            validationQuery=\"SELECT 1\" 
            validationInterval=\"30000\" 
            timeBetweenEvictionRunsMillis=\"30000\"
            maxActive=\"80\" 
            minIdle=\"10\" 
            maxWait=\"10000\" 
            initialSize=\"10\" 
            removeAbandonedTimeout=\"1000\"
            removeAbandoned=\"true\" 
            abandonWhenPercentageFull=\"75\" 
            logAbandoned=\"true\" 
            minEvictableIdleTimeMillis=\"30000\" 
            jmxEnabled=\"true\" 
            jdbcInterceptors=\"org.apache.tomcat.jdbc.pool.interceptor.ConnectionState; org.apache.tomcat.jdbc.pool.interceptor.StatementFinalizer; org.apache.tomcat.jdbc.pool.interceptor.ResetAbandonedTimer; org.apache.tomcat.jdbc.pool.interceptor.SlowQueryReport(threshold=1500)\" 
            username=\"{}\" 
            password=\"{}\" 
            driverClassName=\"com.mysql.jdbc.Driver\" 
            defaultTransactionIsolation=\"READ_COMMITTED\" 
            connectionProperties=\"useSSL=false\" 
            url=\"jdbc:mysql://{}:{}/{}\"/>
        </Context>
        ",mysql_user, mysql_passwd, host, port, db);
        //url=\"jdbc:mysql://localhost:3306/scadalts\"/>

    fs::write(&dest_path, context)?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = App::new("Installer Scada-LTS")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about("This is to prepare the environment to run Scada-LTS")
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
                          .get_matches();

    //TODO welcome installer .. Scada-log_syntax!()
    // remove no-extractr file
    println!(
        "Start inst v{} for Scada-LTS v2.6.10",
        &env!("CARGO_PKG_VERSION")
    );

    let mysql_user = matches.value_of("mysql_user").unwrap_or("root");
    let mysql_password = matches.value_of("mysql_password").unwrap_or("root");
    let mysql_host = matches.value_of("mysql_host").unwrap_or("localhost");
    let mysql_port = matches.value_of("mysql_port").unwrap_or("3306");
    let mysql_db = matches.value_of("mysql_database").unwrap_or("scadalts");

    let ask_for_mysql_password = matches.index_of("ask_for_mysql_password");
    
    println!("MySql user: {}", mysql_user);
    println!("MySql password: {}", mysql_password);
    println!("Mysql host: {}", mysql_host);
    println!("Mysql port: {}", mysql_port);
    println!("Mysql database: {}", mysql_db);
    //if ()
    println!("MySql ask for mysql password: {:?}", ask_for_mysql_password);

    if ask_for_mysql_password.is_some() {
        print!("Type a password for MySql user:{} - password: ", mysql_user);
        std::io::stdout().flush().unwrap();
        let mysql_password = read_password().unwrap();
        println!("The password is: '{}'", mysql_password);
    }

    //Embeded sql not now
    //let mysql_5_7_x86 = String::from("https://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.36-linux-glibc2.12-x86_64.tar.gz");
    //https://dev.mysql.com/doc/refman/5.7/en/windows-create-option-file.html

    //---
    let mut to_fetch_and_unpacking: Vec<Uncopresed> = Vec::new();
    if cfg!(target_os = "windows") {
        to_fetch_and_unpacking.push(
        Uncopresed{
            featch: Featch{ url: String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x86-32_windows_hotspot_11.0.13_8.zip"),
                            file_name: String::from("java.zip")
                          },
            msg: String::from("Get java and unpacking")
        });
        to_fetch_and_unpacking.push(
        Uncopresed{
            featch: Featch{ url: String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.zip"),
                            file_name: String::from("tomcat.zip"),
                          },
            msg: String::from("Get tomcat and unpacking")
        });
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        to_fetch_and_unpacking.push(
            Uncopresed{
                featch: Featch{ url: String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz"),
                                file_name: String::from("java.tar.gz")
                              },
                msg: String::from("Get java and unpacking")
            });
        to_fetch_and_unpacking.push(
            Uncopresed{
                featch: Featch{
                      url: String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.tar.gz"),
                      file_name: String::from("tomcat.tar.gz"),
                    },
                msg: String::from("Get tomcat and unpacking")
            });
    }
    fetch_and_uncopresed(to_fetch_and_unpacking).await.unwrap();

    //---
    let mut to_fetch: Vec<Move> = Vec::new();

    let dir_webapps = format!("./{}/webapps", &DIR_TOMCAT_UNCONPRESED);
    //let dir_cfg = format!("./{}/conf", &DIR_TOMCAT_UNCONPRESED);

    to_fetch.push(
    Move{
        featch: Featch{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/releases/download/v2.6.10-rc1/Scada-LTS.war"),
            file_name: String::from("ScadaBR.war")
        },
        msg: String::from("Get Scada-LTS - and move to tomcat as ScadaBR.war"),
        to_dir: dir_webapps
    });
    // to_fetch.push(
    // Move{
    //     featch: Featch{
    //         url: String::from("https://github.com/SCADA-LTS/installer/releases/download/rv0.0.1/context.xml"),
    //         file_name: String::from("context.xml")
    //     },
    //     msg: String::from("Get config context.xml - and move to tomcat"),
    //     to_dir: dir_cfg
    // });
    to_fetch.push(
    Move{
        featch: Featch{
            url: String::from("https://repo1.maven.org/maven2/mysql/mysql-connector-java/5.1.49/mysql-connector-java-5.1.49.jar"),
            file_name: String::from("mysql_connector.jar")
        },
        msg: String::from("Get lib - mysql-connector-java-5.1.49.jar - and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(Move {
        featch: Featch {
            url: String::from(
                "https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/activation.jar",
            ),
            file_name: String::from("activation.jar"),
        },
        msg: String::from("Get lib - activation.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED),
    });
    to_fetch.push(Move{
        featch: Featch{ url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-api-2.4.0-b180830.0359.jar"),
                        file_name: String::from("jaxb-api-2.4.0-b180830.0359.jar")
                       },
        msg: String::from("Get lib - jaxb-api-2.4.0-b180830.0359.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(Move {
        featch: Featch {
            url: String::from(
                "https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-core-3.0.2.jar",
            ),
            file_name: String::from("jaxb-core-3.0.2.jar"),
        },
        msg: String::from("Get lib - jaxb-core-3.0.2.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED),
    });
    to_fetch.push(
    Move{
        featch: Featch {
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-runtime-2.4.0-b180830.0438.jar"),
            file_name: String::from("jaxb-runtime-2.4.0-b180830.0438.jar")
        },
        msg: String::from("Get lib - jaxb-runtime-2.4.0-b180830.0438.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });

    fetch_and_move(to_fetch).await.unwrap();

    //correct configuration
    //write_user_and_passwd_mysql(mysql_user, mysql_password).await.unwrap();

    //create configuration
    create_config_xml(mysql_user, mysql_password, mysql_host, mysql_port, mysql_db ).await.unwrap();

    //usuniecie dodakowych aplikacji menager itp
    //wylaczenie portu 8005
    println!("If you have installed and running mysql server on localhost:3603 with user {} and {} {} and the scadalts database is set up then you can run the \"./start.sh\" program",mysql_user,mysql_password,mysql_user);
    println!("Start in webrowser - http://localhost:8080/ScadaBR");

    //---
    println!("end");
}