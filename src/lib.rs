use serde::{Deserialize, Serialize};
pub mod local_file;
pub mod web_page;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: String,
    pub path: String,
}
impl Config {
    pub fn new<T: ToString>(host: T, port: T, path: T) -> Config {
        Config {
            server: Server {
                host: host.to_string(),
                port: port.to_string(),
                path: path.to_string(),
            },
        }
    }
}

pub async fn parse_args() -> Config {
    let args = clap::Command::new("file_server")
        .author("ibukifuko")
        .version("0.1.0")
        .about("A simple file server written in rust")
        .arg(
            clap::Arg::new("host")
                .long("host")
                .default_value("0.0.0.0")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("port")
                .long("port")
                .default_value("8080")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("path")
                .long("path")
                .default_value("./")
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            clap::Arg::new("config")
                .short('c')
                .long("config")
                .value_parser(clap::value_parser!(String))
                .conflicts_with_all(["host", "port", "path"]),
        )
        .get_matches();
    let config: Option<&String> = args.get_one("config");
    match config {
        None => {
            let host: &String = args.get_one("host").unwrap();
            let port: &String = args.get_one("port").unwrap();
            let path: &String = args.get_one("path").unwrap();
            if path.ends_with("/") {
                Config::new(host, port, path)
            } else {
                let path = format!("{path}/");
                Config::new(host, port, &path)
            }
        }
        Some(config) => {
            let config_content = tokio::fs::read_to_string(config)
                .await
                .expect("config file can't be opened,");
            let mut config: Config = toml::from_str(&config_content).expect("parse config failed");
            if config.server.path.ends_with("/") {
                config
            } else {
                config.server.path.push('/');
                config
            }
        }
    }
}
