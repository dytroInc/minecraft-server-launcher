use tokio::fs;
use std::path::Path;

pub struct ConfigFileData {
    pub server_directory_name: String,
    pub server: String,
    pub memory: u8,
    pub backup: bool,

    pub max_players: u16,
    pub plugins_directory_name: String,
    pub worlds_directory_name: String,
    pub port: u16,
    pub default_plugins: Vec<String>
}

impl Default for ConfigFileData {
    fn default() -> Self {
        ConfigFileData {
            server_directory_name: String::from("server"),
            server: String::from("paper-1.19.1-latest"),
            memory: 4,
            backup: false,

            max_players: 100,
            plugins_directory_name: String::from("plugins"),
            worlds_directory_name: String::from("worlds"),
            port: 25565,
            default_plugins: vec![String::from("https://github.com/monun/auto-reloader/releases/download/0.0.5/auto-reloader-0.0.5.jar")]
        }
    }
}

impl ToString for ConfigFileData {
    fn to_string(&self) -> String {
        format!(
            "\
            server_directory_name={}\n\
            server={}\n\
            memory={}\n\
            backup={}\n\
            max_players={}\n\
            plugins_directory_name={}\n\
            worlds_directory_name={}\n\
            port={}\n\
            default_plugins=[{}]\
            ",
            self.server_directory_name, self.server, self.memory, self.backup, self.max_players,
            self.plugins_directory_name, self.worlds_directory_name, self.port, self.default_plugins.join(", ")
        )
    }
}

pub(crate) async fn fetch_data(config_file: &Path) -> ConfigFileData {
    let raw_data = if config_file.is_file() {
        fs::read_to_string(config_file).await.expect("An error has occurred while reading the configuration file.")
    } else {
        String::from("")
    };

    let mut data = ConfigFileData::default();

    for line in raw_data.split("\n") {
        if let Some((key, value)) = line.split_once("=") {
            match key {
                "server_directory_name" => data.server_directory_name = String::from(value),
                "server" => data.server = String::from(value),
                "plugins_directory_name" => data.plugins_directory_name = String::from(value),
                "worlds_directory_name" => data.worlds_directory_name = String::from(value),

                "memory" => data.memory = value.parse().expect("An error has occurred while parsing a configuration value."),
                "max_players" => data.max_players = value.parse().expect("An error has occurred while parsing a configuration value."),
                "port" => data.port = value.parse().expect("An error has occurred while parsing a configuration value."),

                "backup" => data.backup = value == "true",

                "default_plugins" => {
                    if value.starts_with("[") && value.ends_with("]") {
                        data.default_plugins = (&value[1..value.len() - 1]).split(", ").map(|v| String::from(v)).collect()
                    }
                }
                _ => {}
            }
        }
    }

    data
}