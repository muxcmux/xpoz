use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use shellexpand::tilde;
use std::env::args;

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub public_dir: String,
    pub graphiql: bool,
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

impl App {
    pub fn database_url(&self) -> String {
        format!("sqlite://{}", tilde(&self.database))
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
        format!("sqlite://{}", tilde(&self.database))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub photos: Photos,
    pub server: Server,
    pub app: App,
}

fn set_defaults(config: &mut Config) {
    let defaults = [
        ["server.address", "0.0.0.0:1234"],
        ["server.public_dir", "./public"],
        ["server.graphiql", "true"],
        ["photos.library", "~/Pictures/Photos Library.photoslibrary"],
        [
            "photos.database",
            "~/Pictures/Photos Library.photoslibrary/database/Photos.sqlite",
        ],
        ["photos.originals", "originals"],
        ["photos.renders", "resources/renders"],
        ["photos.resized", "resources/derivatives"],
        ["photos.thumbs", "resources/derivatives/masters"],
        ["app.database", "xpoz.sqlite"],
    ];

    for s in defaults.iter() {
        config.set_default(s[0], s[1]).expect("Config error");
    }
}

impl Settings {
    pub fn from_file(filename: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        set_defaults(&mut config);
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
