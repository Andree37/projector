use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}

pub struct Projector {
    config: Config,
    data: Data,
}

fn default_data() -> Data {
    return Data {
        projector: HashMap::new(),
    };
}

impl Projector {
    fn FromConfig(config: Config) -> Self {
        if std::fs::metadata(&config.config).is_ok() {
            let contents = std::fs::read_to_string(&config.config);
            let contents = contents.unwrap_or("{\"projector\":{}}".to_string());
            let data = serde_json::from_str(&contents).unwrap_or(default_data());

            return Projector { config, data };
        }
        return Projector {
            config,
            data: default_data(),
        };
    }
}
