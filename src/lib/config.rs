use clap::Parser;
use eyre::*;
use serde::de::DeserializeOwned;
use serde::*;
use serde_json::Value;
use std::env::current_dir;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CliArgument {
    /// The path to config file
    #[clap(
        short,
        long,
        value_parser,
        value_name = "FILE",
        default_value = "etc/config.json",
        env = "CONFIG"
    )]
    config: PathBuf,
    /// The path to config file
    #[clap(short, long)]
    config_entry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsServerConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: u16,
    #[serde(default)]
    pub pub_certs: Option<Vec<String>>,
    #[serde(default)]
    pub priv_cert: Option<String>,
    #[serde(default)]
    pub debug: bool,
    #[serde(skip)]
    pub header_only: bool,
}

pub fn load_config<Config: DeserializeOwned + Debug>(mut service_name: String) -> Result<Config> {
    let args: CliArgument = CliArgument::parse();

    println!("Working directory {}", current_dir()?.display());
    println!("Loading config from {}", args.config.display());
    let config = std::fs::read_to_string(&args.config)?;
    let mut config: Value = serde_json::from_str(&config)?;
    if let Some(entry) = args.config_entry {
        service_name = entry;
    }
    let service_config = config
        .get_mut(&service_name)
        .ok_or_else(|| eyre!("Service {} not found in config", service_name))?
        .clone();
    let root = config.as_object_mut().unwrap();
    for (k, v) in service_config.as_object().unwrap() {
        root.insert(k.clone(), v.clone());
    }
    root.remove(&service_name);
    let config: Config = serde_json::from_value(config)?;
    println!("App config {:#?}", config);
    Ok(config)
}
