use directories::BaseDirs;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

lazy_static! {
    pub static ref SEARCH_CONFIG: SearchConfig = load_config();
}

fn load_config() -> SearchConfig {
    // check if the configuration file exists.
    if let Some(base_dirs) = BaseDirs::new() {
        let dir = base_dirs.config_dir().to_path_buf();
        let conf_file: PathBuf = dir.join("hors").join("config.toml");
        if !conf_file.exists() {
            Default::default()
        } else {
            toml::from_str(&fs::read_to_string(conf_file).unwrap_or_default()).unwrap_or_default()
        }
    } else {
        SearchConfig::default()
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchConfig {
    engine_domain: EngineDomain,
}

impl SearchConfig {
    pub fn get_bing_domain(&self) -> &str {
        &self.engine_domain.bing
    }

    pub fn get_ddg_domain(&self) -> &str {
        &self.engine_domain.duckduckgo
    }

    pub fn get_google_domain(&self) -> &str {
        &self.engine_domain.google
    }
}

#[derive(Deserialize, Debug)]
pub struct EngineDomain {
    #[serde(default = "ddg_default")]
    duckduckgo: String,
    #[serde(default = "bing_default")]
    bing: String,
    #[serde(default = "google_default")]
    google: String,
}

impl Default for SearchConfig {
    fn default() -> SearchConfig {
        SearchConfig {
            engine_domain: Default::default(),
        }
    }
}

impl Default for EngineDomain {
    fn default() -> EngineDomain {
        EngineDomain {
            duckduckgo: ddg_default(),
            bing: bing_default(),
            google: google_default(),
        }
    }
}

fn ddg_default() -> String {
    "duckduckgo.com".to_string()
}

fn bing_default() -> String {
    "www.bing.com".to_string()
}

fn google_default() -> String {
    "www.google.com".to_string()
}
