use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;
use crate::ext::ExpectExt;
use shellexpand::tilde;

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Photos {
    pub library: String,
    pub database: String,
    pub originals: String,
    pub renders: String,
    pub resized: String,
    pub thumbs: String
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
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub photos: Photos,
    pub server: Server
}

fn set_defaults(config: &mut Config) {
    let defaults = [
        ["server.address", "0.0.0.0:1234"],
        ["photos.library", "~/Pictures/Photos Library.photoslibrary"],
        ["photos.database", "test.sqlite"],
        ["photos.originals", "originals"],
        ["photos.renders", "resources/renders"],
        ["photos.resized", "resources/derivatives"],
        ["photos.thumbs", "resources/derivatives/masters"]
    ];

    for s in defaults.iter() {
        config.set_default(s[0], s[1]).expect_or_exit("Config error");
    }
}

impl Settings {
    pub fn from_file(filename: &str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        set_defaults(&mut config);
        config.merge(File::with_name(filename).required(false))?;
        config.merge(Environment::with_prefix("xpoz").separator("__"))?;
        config.try_into()
    }

    pub fn default_file() -> &'static str { "settings.yml" }
}
