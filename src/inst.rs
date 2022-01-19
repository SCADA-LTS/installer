
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;
use tar::Archive;
use std::ffi::OsStr;
use std::io;
use flate2::read::{GzDecoder};
use std::process::{Command};

//use std::process::Command;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


///Featch and Uncopresed - The structure needed to take a file from the `url` resource and save it as `file_name` and unpack it in the current directory, and display the `msg` information
pub struct Uncopresed {
    ///Structure needed to retrieve a file from resource `featch`
    pub featch: Featch,
    ///Display the `msg` information
    pub msg: &'static str,
}

///Featch - Structure needed to retrieve a file from resource `url` and save as `file_name`
pub struct Featch {
    ///Retrive a file from resource `url`
    pub url: &'static str,
    ///Save as `file_name`
    pub file_name: &'static str,
}

/// Decoding type
enum Decoder {
    Zip,
    Gz,
}

///Featch and Move - Structure needed to take a file from the resource `url` and save it as `file_name` and move it to the specified directory - `to_dir`, and display the `msg` information
pub struct Move {
    ///Structure needed to retrieve a file from resource `featch`
    pub featch: Featch,
    ///Display the `msg` information
    pub msg: &'static str,
    ///Move it to the specified directory `to_dir`
    pub to_dir: &'static str,
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
    let response = reqwest::get(url).await?; //request
    let mut file = File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

/// unzip 
///
/// # Arguments
///
/// * `file_name` - A String - to unzip
///
/// # Examples
///
/// ```
/// // You can have rust code between fences inside the comments
/// // If you pass --test to `rustdoc`, it will even test it for you!
///
/// unzip("java.zip");
/// ```
async fn unzip(file_name: &str) -> Result<()> {
    
    let fname = std::path::Path::new(file_name);
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
    
    let from = Path::new("./").join(&file_name);
    let to = Path::new(&to_dir).join(&file_name);
    println!("move file-name: {:?}, to_dir: {:?}", &from, &to);
    fs::rename(&from, &to).unwrap();
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
pub async fn fetch_and_move(to_fetch: Vec<Move>) -> Result<()> {
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
pub async fn fetch_and_uncopresed(to_fetch: Vec<Uncopresed>) -> Result<()> {
    for u in to_fetch {
        println!("{}", &u.msg);
        fetch_url(&u.featch.url, &u.featch.file_name).await.unwrap();
        let exten = Path::new(&u.featch.file_name).extension().and_then(OsStr::to_str).unwrap();
        
        if exten == "zip" {
            //println!("uncomprese: zip extend: {}", exten);
            uncopresed(&u.featch.file_name, Decoder::Zip).await.unwrap();
        } else {
            //println!("uncomprese: gz extend: {}", exten);
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
pub async fn create_config_xml(mysql_user: &str, mysql_passwd: &str, host: &str, port: &str, db: &str, dir_tomcat_uncompresed: &str) -> Result<()> {
    let out_dir = format!("./{}/conf", &dir_tomcat_uncompresed);
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

    println!("Set configuration context.xml");
    Ok(())
}

pub async fn sh(script: &str) {
    println!("Start: {}", script);
    Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute process");
}

pub fn sh_n(script: &str) {
    println!("Start: {}", script);
    Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to execute process");
}


