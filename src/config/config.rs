use std::io::Write;
use std::path::{Path, PathBuf};

use termcolor;
use termcolor::WriteColor;
use tokio::fs;

use crate::{error, info};
use crate::config::config_file::{ConfigFileData, fetch_data};
use crate::helpers::web_data::{BuildData, VersionData};
use crate::helpers::web_helpers::{download_file_from_url, get_request};

pub async fn setup_server_from_config() -> Result<(PathBuf, ConfigFileData), ()> {
    let config_file = Path::new("server.properties");

    let data = fetch_data(&config_file).await;

    fs::write(config_file, &data.to_string()).await.expect("An error has occurred while writing config data to the configuration file.");

    let server_dir = &data.server_directory_name;

    let path = Path::new(&server_dir);
    if !path.exists() {
        fs::create_dir(path).await.expect("An error has occurred while creating the server directory.");
    }

    let server_data: &Vec<&str> = &data.server.split("-").collect();

    let jar = match server_jar(&server_data).await {
        Ok(path) => path,
        Err(msg) => {
            error!("{}", msg);
            return Err(());
        }
    };

    info!("Found server jar file: {:?}", jar);

    if server_data[0] != "vanilla" {
        let dirname = format!("{}/{}", server_dir, &data.plugins_directory_name);
        let plugins_dir = Path::new(&dirname);

        if !plugins_dir.exists() {
            fs::create_dir(plugins_dir).await.expect("An error has occurred while creating the server directory.");
        }

        let plugins = &data.default_plugins;

        if !plugins.is_empty() {
            for (index, plugin) in plugins.iter().enumerate() {
                let name = plugin.split("/").last().unwrap();
                info!("Downloading plugin \"{}\"... ({}/{})", name, index + 1, plugins.len());
                download_file_from_url(format!("{}/{}", dirname, name), plugin.clone())
                    .await.expect("An error has occurred while downloading a default plugin.");
            }

            info!("Downloaded all default plugin jars!");
        }
    }

    Ok((jar, data))
}

async fn server_jar(data: &Vec<&str>) -> Result<PathBuf, String> {
    if data.len() == 3 {
        let repo = match home::home_dir() {
            Some(path_buf) => {
                let buf = &path_buf;
                buf.join(".mcservers").join(data[0])
            }
            None => {
                return Err(String::from("Couldn't find the home directory! Exiting server setup..."));
            }
        };

        if !repo.exists() {
            println!("{}", repo.display());

            if let Err(_) = fs::create_dir_all(&repo).await {
                return Err(String::from("Couldn't create the .mcserver directory! Exiting server setup..."));
            };
        }

        let version = data[1];
        let mut build = data[2].to_owned();

        match data[0] {
            "paper" => {
                if build == "latest" {
                    let paper = match get_request(&format!("https://papermc.io/api/v2/projects/paper/versions/{version}")).await {
                        Ok(response) => response,
                        Err(_) => return Err(String::from("Couldn't get the response of the GET request! Exiting server setup..."))
                    };

                    let json: Result<VersionData, _> = serde_json::from_str(&paper);
                    match json {
                        Ok(value) => build = value.builds.last().unwrap().to_string(),
                        Err(_) => return Err(String::from("Failed to parse the string to json! Exiting server setup..."))
                    }
                }

                let build_url = format!("https://papermc.io/api/v2/projects/paper/versions/{version}/builds/{build}");

                let raw_build_data = match get_request(&build_url).await {
                    Ok(response) => response,
                    Err(_) => return Err(String::from("Couldn't get the response of the GET request! Exiting server setup..."))
                };

                let json: Result<BuildData, _> = serde_json::from_str(&raw_build_data);

                let file_name = match json {
                    Ok(value) => value.downloads.application.name,
                    Err(_) => return Err(String::from("Failed to parse the string to json! Exiting server setup..."))
                };

                let jar = repo.join(&file_name);

                if !jar.exists() {
                    if !download_file_from_url(jar.to_str().unwrap().to_owned(), format!("{}/{}/{}", build_url, "downloads", file_name)).await.is_ok() {
                        return Err(String::from("Failed to download the server jar file! Exiting server setup..."));
                    }
                }

                Ok(jar)
            }
            _ => Err(String::from("Unknown server type! Exiting server setup..."))
        }
    } else {
        Err(String::from("Incorrect server format! Exiting server setup..."))
    }
}