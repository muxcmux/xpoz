use config::{ConfigError, Config, File, Environment};
use serde::Deserialize;
use crate::ext::ExpectExt;

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Photos {
    pub library: String,
    pub database: String
}

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub photos: Photos,
    pub server: Server
}

fn set_defaults(config: &mut Config) {
    let defaults = [
        ["server.address", "127.0.0.1:1234"],
        ["photos.library", "~/Pictures/Photos Library.photoslibrary"],
        ["photos.database", "test.sqlite"]
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
