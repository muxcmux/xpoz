use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;
use shellexpand::tilde;
use std::env::args;

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub public_dir: String,
    pub index_file: String,
    pub graphiql: bool,
    pub ssl: bool,
    pub cert: String,
    pub key: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Photos {
    pub library: String,
    pub database: String,
    pub originals: String,
    pub renders: String,
    pub resized: String,
    pub thumbs: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct App {
    pub database: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Media {
    pub transcode_videos: bool,
    pub ffmpeg: FFmpeg,
    pub workers: usize,
    pub videos_path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FFmpeg {
    pub bin: String,
    pub probe: String,
    pub sdr: Vec<String>,
    pub hdr: Vec<String>,
}

impl App {
    pub fn database_url(&self) -> String {
        format!("{}", tilde(&self.database))
    }
}

impl Photos {
    fn dir_to(&self, subdir: &String) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::new();
        path.push(tilde(&self.library).to_string());
        path.push(subdir);
        path
    }

    pub fn originals_dir(&self) -> std::path::PathBuf {
        self.dir_to(&self.originals)
    }

    pub fn renders_dir(&self) -> std::path::PathBuf {
        self.dir_to(&self.renders)
    }

    pub fn resized_dir(&self) -> std::path::PathBuf {
        self.dir_to(&self.resized)
    }

    pub fn thumbs_dir(&self) -> std::path::PathBuf {
        self.dir_to(&self.thumbs)
    }

    pub fn database_url(&self) -> String {
        format!("{}", tilde(&self.database))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub photos: Photos,
    pub server: Server,
    pub app: App,
    pub media: Media,
}

impl Settings {
    pub fn from_file(filename: &str) -> Result<Self, ConfigError> {
        let default = String::from_utf8_lossy(include_bytes!("default_config.yml"));
        let mut config = Config::new();
        config.merge(File::from_str(&default, FileFormat::Yaml))?;
        config.merge(File::with_name(filename).required(false))?;
        config.merge(Environment::with_prefix("XPOZ").separator("__"))?;
        config.try_into()
    }

    pub fn default_file() -> &'static str {
        "settings.yml"
    }
}

pub fn load_settings() -> Settings {
    let config_file = args()
        .nth(1)
        .unwrap_or_else(|| Settings::default_file().to_string());
    Settings::from_file(&config_file).expect("Config error")
}
