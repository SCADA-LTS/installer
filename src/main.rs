use tar::Archive;
use std::{io::Cursor};
use flate2::read::{GzDecoder};
use tokio::process::Command;
use std::fs::File;
use clap::{App, Arg};
use rpassword::read_password;
use std::io::Write;
//use quick_xml::Writer;
//use quick_xml::Reader;
//use quick_xml::events::{Event, BytesEnd, BytesStart};
use std::env;
use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
const DIR_TOMCAT_UNCONPRESED:&str = "apache-tomcat-9.0.48";

//Featch
struct Featch{
    url: String,
    file_name: String
}

//Featch and Uncopresed
struct Uncopresed{
    featch: Featch,
    msg: String
}

//Featch and Move
struct Move{
    featch: Featch,
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

async fn move_file(file_name: &str, to_dir: &str) {
    if cfg!(target_os = "windows") {
        cmd("move", vec![&file_name, &to_dir],"./").await.unwrap();
    } else if cfg!(target_os = "linux") {
        let args_sh_move = vec![ "-f", &file_name, &to_dir];
        cmd("mv", args_sh_move,"./").await.unwrap();
    }
}

async fn fetch_and_move(to_fetch:Vec<Move>) -> Result<()>{
    for m in to_fetch {
        println!("{}",&m.msg);
        fetch_url(&m.featch.url, &m.featch.file_name).await.unwrap();
        move_file(&m.featch.file_name, &m.to_dir).await;
    }
    Ok(())
}

async fn fetch_and_uncopresed(to_fetch:Vec<Uncopresed>) -> Result<()>{
    for u in to_fetch {
        println!("{}", &u.msg);
        fetch_url(&u.featch.url, &u.featch.file_name).await.unwrap();
        uncopresed(&u.featch.file_name).await.unwrap();
    }
    Ok(())
}

// async fn write_user_and_passwd_mysql(user:&str, passwd:&str) ->Result<()> {
//     let filename = format!("./{}/conf/context.xml",DIR_TOMCAT_UNCONPRESED);
//     println!("read file - {}", filename);
//     let xml = fs::read_to_string(&filename)
//     .expect("Something went wrong reading the file");

//     let mut reader = Reader::from_str(&xml);
//     reader.trim_text(true);
//     let mut writer = Writer::new(Cursor::new(Vec::new()));
//     let mut buf = Vec::new();
//     loop {
//         match reader.read_event(&mut buf) {
//             Ok(Event::Start(ref e)) if e.name() == b"Resource" => {
    
//                 // crates a new element ... alternatively we could reuse `e` by calling
//                 // `e.into_owned()`
//                 let mut elem = BytesStart::owned(b"Resource".to_vec(), "Resource".len());
    
//                 // collect existing attributes
//                 elem.extend_attributes(e.attributes().map(|attr| attr.unwrap()));
    
//                 // copy existing attributes, adds a new my-key="some value" attribute
//                 elem.push_attribute(("username", user));
//                 elem.push_attribute(("password", passwd));
            
        
    
//                 // writes the event to the writer
//                 assert!(writer.write_event(Event::Start(elem)).is_ok());
               
//             },
//             Ok(Event::End(ref e)) if e.name() == b"Resource" => {
//                 assert!(writer.write_event(Event::End(BytesEnd::borrowed(b"Resource"))).is_ok());
//             },
//             Ok(Event::Eof) => break,
//             Ok(e) => assert!(writer.write_event(e).is_ok()),
//             // or using the buffer
//             // Ok(e) => assert!(writer.write(&buf).is_ok()),
//             Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
//         }
//         buf.clear();
        
//     }
    
//     let result = writer.into_inner().into_inner();   
//     let context = String::from_utf8_lossy(&result);
//     println!("do zapisania {}", &context ); 
//     fs::write(&filename, &context.to_string()).unwrap();

//     // let result_xml = fs::read_to_string(&filename)
//     // .expect("Something went wrong reading the file");

//     // println!("xml:\n{}", result_xml);


//     Ok(())

// }
                                                                  
async fn create_config_xml(mysql_user:&str, mysql_passwd:&str) -> Result<()>{

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
            url=\"jdbc:mysql://localhost:3306/scadalts\"/>
        </Context>
        ",mysql_user, mysql_passwd);

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
                          .arg(Arg::with_name("ask_for_mysql_password")
                               .short("a")
                               .long("ask_for_mysql_paswd")
                               .help("Ask for the mysql password to be used by Scada-LTS \n to connect to the mysql server")
                            )
                          .get_matches();

    //TODO welcome installer .. Scada-log_syntax!()
    // remove no-extractr file
    println!("Start inst v{} for Scada-LTS v2.6.10", &env!("CARGO_PKG_VERSION"));

    

    let mysql_user = matches.value_of("mysql_user").unwrap_or("root");
    let mysql_password = matches.value_of("mysql_password").unwrap_or("root");
    let ask_for_mysql_password = matches.index_of("ask_for_mysql_password");
    println!("MySql user: {}", mysql_user);
    println!("MySql password: {}", mysql_password);
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
            featch: Featch{ 
                  url: String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x86-32_windows_hotspot_11.0.13_8.zip"),
                  file_name: String::from("java.tar.gz")
               },
            msg: String::from("Get java and unpacking")
        });
        to_fetch_and_unpacking.push(
        Uncopresed{
            featch: Featch{
                  url: String::from("https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.48/bin/apache-tomcat-9.0.48.zip"),
                  file_name: String::from("tomcat.tar.gz"),
                },
            msg: String::from("Get tomcat and unpacking")
        });
    } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        to_fetch_and_unpacking.push(
            Uncopresed{
                featch: Featch{ 
                      url: String::from("https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz"),
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
    to_fetch.push(
    Move{
        featch: Featch{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/activation.jar"),
            file_name: String::from("activation.jar")
        },
        msg: String::from("Get lib - activation.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(    
    Move{
        featch: Featch{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-api-2.4.0-b180830.0359.jar"),
            file_name: String::from("jaxb-api-2.4.0-b180830.0359.jar")
        },
        msg: String::from("Get lib - jaxb-api-2.4.0-b180830.0359.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
    });
    to_fetch.push(
    Move{
        featch: Featch{
            url: String::from("https://github.com/SCADA-LTS/Scada-LTS/raw/develop/tomcat/lib/jaxb-core-3.0.2.jar"),
            file_name: String::from("jaxb-core-3.0.2.jar")
        },
        msg: String::from("Get lib - jaxb-core-3.0.2.jar and move to tomcat"),
        to_dir: format!("./{}/lib", DIR_TOMCAT_UNCONPRESED)
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
    create_config_xml(mysql_user, mysql_password).await.unwrap();

    //usuniecie dodakowych aplikacji menager itp
    //wylaczenie portu 8005
    println!("If you have installed and running mysql server on localhost:3603 with user {} and {} {} and the scadalts database is set up then you can run the \"./start.sh\" program",mysql_user,mysql_password,mysql_user);
    println!("Start in webrowser - http://localhost:8080/ScadaBR");

    //---
    println!("end");
}

