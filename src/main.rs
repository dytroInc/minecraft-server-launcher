use std::{env};
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

use termcolor;
use termcolor::WriteColor;

use tokio::fs;

use config::config::setup_server_from_config;
use minecraft_server_launcher::{config, error, info, reset};
use minecraft_server_launcher::config::config_file::ConfigFileData;
use minecraft_server_launcher::helpers::java_helpers::find_java_executable;

use chrono::offset::Local;
use tar::Builder;

#[tokio::main]
async fn main() {
    info!("Starting server...");

    if let Ok((jar, config)) = setup_server_from_config().await {
        let ConfigFileData { memory, server_directory_name, max_players, plugins_directory_name, worlds_directory_name, port, .. } = &config;

        let memory_x = format!("-Xmx{}G", memory);
        let memory_s = format!("-Xms{}G", memory);

        let mut jvm_arguments = vec![
            memory_x.as_str(),
            memory_s.as_str(),
            "-XX:+ParallelRefProcEnabled",
            "-XX:MaxGCPauseMillis=200",
            "-XX:+UnlockExperimentalVMOptions",
            "-XX:+DisableExplicitGC",
            "-XX:+AlwaysPreTouch",
            "-XX:G1HeapWastePercent=5",
            "-XX:G1MixedGCCountTarget=4",
            "-XX:G1MixedGCLiveThresholdPercent=90",
            "-XX:G1RSetUpdatingPauseTimePercent=5",
            "-XX:SurvivorRatio=32",
            "-XX:+PerfDisableSharedMem",
            "-XX:MaxTenuringThreshold=1",
            "-Dusing.aikars.flags=https://mcflags.emc.gs",
            "-Daikars.new.flags=true",
            "-Dfile.encoding=UTF-8",
            "-Dcom.mojang.eula.agree=true",
        ];

        jvm_arguments.append(&mut if memory < &12 {
            vec![
                "-XX:G1NewSizePercent=30",
                "-XX:G1MaxNewSizePercent=40",
                "-XX:G1HeapRegionSize=8M",
                "-XX:G1ReservePercent=20",
                "-XX:InitiatingHeapOccupancyPercent=15",
            ]
        } else {
            vec![
                "-XX:G1NewSizePercent=40",
                "-XX:G1MaxNewSizePercent=50",
                "-XX:G1HeapRegionSize=16M",
                "-XX:G1ReservePercent=15",
                "-XX:InitiatingHeapOccupancyPercent=20",
            ]
        });

        let s_arg = format!("-s{}", max_players);
        let plugins_arg = format!("-P{}", plugins_directory_name);
        let worlds_arg = format!("-W{}", worlds_directory_name);
        let port_arg = format!("-p{}", port);

        jvm_arguments.append(&mut vec![
            "-jar",
            jar.to_str().unwrap(),
            "--nogui",
            s_arg.as_str(),
            plugins_arg.as_str(),
            worlds_arg.as_str(),
            port_arg.as_str(),
        ]);

        match find_java_executable() {
            Ok(java) => {
                let server_path = Path::new(server_directory_name);
                let server_directory = read_dir(server_path).unwrap();
                if let Err(_) = env::set_current_dir(server_path) {
                    error!("Something wrong has happened while changing the current directory! Exiting server setup...");
                    return;
                }
                reset!();

                Command::new(&java)
                    .args(&jvm_arguments)
                    .stdout(Stdio::inherit())
                    .spawn()
                    .unwrap().wait().unwrap();

                if config.backup {
                    info!("Starting the backup process...");
                    let date = Local::now();

                    let backup_dirname = ".backup";
                    let backup_dir = Path::new(backup_dirname);
                    if !backup_dir.exists() {
                        if let Err(_) = fs::create_dir(backup_dir).await {
                            error!("An error has occurred while creating the .backup directory! Aborting the backup process...");
                            return;
                        }
                    }

                    let backup_file = File::create(format!("{}/backup-{}.tar.gz", backup_dirname, date.format("%Y%m%d-%H%M%S"))).unwrap();
                    let mut builder = Builder::new(backup_file);

                    for server_content_path in server_directory {
                        let unwrapped = &server_content_path.unwrap();

                        if unwrapped.file_name() == backup_dirname {
                            continue;
                        } else if let Err(error_desc) = builder.append_path(Path::new(unwrapped.file_name().to_str().unwrap())) {
                            error!("An error has occurred while archiving the server data! Aborting the backup process... {}", error_desc);
                            return;
                        }
                    }

                    builder.finish().unwrap();
                    info!("Backup completed!")
                }

                info!("Exiting...")
            }
            Err(message) => error!("{}", message)
        };
    }
}