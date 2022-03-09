// if cfg!(target_os = "windows") {
//     to_fetch_and_unpacking.push(
//     Uncopresed{
//         featch: Featch{ url: "https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x86-32_windows_hotspot_11.0.13_8.zip",
//                         file_name: "java.zip"
//                       },
//         msg: "Get java and unpacking"
//     });
//     to_fetch_and_unpacking.push(
//     Uncopresed{
//         featch: Featch{ url: "https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.56/bin/apache-tomcat-9.0.56.zip",
//                         file_name: "tomcat.zip",
//                       },
//         msg: "Get tomcat and unpacking"
//     });
//     to_fetch_and_unpacking.push(Uncopresed {
//         featch: Featch {
//             url: "https://downloads.mysql.com/archives/get/p/23/file/mysql-5.7.35-win32.zip",
//             file_name: "my.zip",
//         },
//         msg: "Get mysql and unpacking",
//     });
// } else if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
//     to_fetch_and_unpacking.push(
//         Uncopresed{
//             featch: Featch{ url: "https://github.com/adoptium/temurin11-binaries/releases/download/jdk-11.0.13%2B8/OpenJDK11U-jdk_x64_linux_hotspot_11.0.13_8.tar.gz",
//                             file_name: "java.tar.gz"
//                           },
//             msg: "Get java and unpacking"
//         });
//     to_fetch_and_unpacking.push(
//         Uncopresed{
//             featch: Featch{
//                   url: "https://archive.apache.org/dist/tomcat/tomcat-9/v9.0.56/bin/apache-tomcat-9.0.56.tar.gz",
//                   file_name: "tomcat.tar.gz",
//                 },
//             msg: "Get tomcat and unpacking"
//         });
//     to_fetch_and_unpacking.push(
//             Uncopresed{
//                 featch: Featch{
//                       url: "https://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.36-linux-glibc2.12-x86_64.tar.gz",
//                       file_name: "my.tar.gz",
//                     },
//                 msg: "Get mysql and unpacking"
//             });
//     //TODO for windows;
//     to_fetch_and_unpacking.push(
//             Uncopresed{
//                 featch: Featch{
//                       url: "https://dev.mysql.com/get/Downloads/MySQL-Shell/mysql-shell-8.0.27-linux-glibc2.12-x86-64bit.tar.gz",
//                       file_name: "myshell.tar.gz",
//                 },
//                 msg: "Get mysql shell and unpacking"
//             });

// }