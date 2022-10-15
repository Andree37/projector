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
    pub fn get_value_all(&self) -> HashMap<&String, &String> {
        let mut paths = vec![];
        let mut curr = Some(self.config.pwd.as_path()); // as path creates a reference to it

        while let Some(p) = curr {
            paths.push(p);
            curr = p.parent();
        }

        let mut out = HashMap::new();
        for path in paths.into_iter().rev() {
            if let Some(map) = self.data.projector.get(path) {
                out.extend(map.iter())
            }
        }

        return out;
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        let mut curr = Some(self.config.pwd.as_path()); // as path creates a reference to it
        let mut out = None;

        while let Some(p) = curr {
            if let Some(dir) = self.data.projector.get(p) {
                if let Some(value) = dir.get(key) {
                    out = Some(value);
                    break;
                }
            }
            curr = p.parent();
        }

        return out;
    }

    pub fn set_value(&mut self, key: String, value: String) {
        self.data.projector.entry(PathBuf::from(&self.config.pwd))
            .or_default()
            .insert(key, value);
    }

    pub fn remove_value(&mut self, key: &str) {
        self.data.projector
            .get_mut(&self.config.pwd)
            .map(|x| {
                x.remove(key);
            });
    }

    pub fn from_config(config: Config) -> Self {
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use collection_macros::hashmap;

    use crate::config::Config;
    use crate::config::Operation;
    use crate::projector::{Data, Projector};

    fn get_data() -> HashMap<PathBuf, HashMap<String, String>> {
        return hashmap! {
            PathBuf::from("/") => hashmap! {
                "foo".into() => "bar1".into(),
                "fem".into() => "is_great".into()
            },
            PathBuf::from("/foo") => hashmap! {
                "foo".into() => "bar2".into()
            },
            PathBuf::from("/foo/bar") => hashmap! {
                "foo".into() => "bar3".into()
            }
        };
    }

    fn get_projector(pwd: PathBuf) -> Projector {
        return Projector {
            config: Config {
                pwd,
                config: PathBuf::from(""),
                operation: Operation::Print(None),
            },
            data: Data {
                projector: get_data()
            },
        };
    }

    #[test]
    fn get_value() {
        let proj = get_projector(PathBuf::from("/foo/bar"));
        assert_eq!(proj.get_value("foo"), Some(&String::from("bar3")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")));
    }

    #[test]
    fn set_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.set_value(String::from("foo"), String::from("bar4"));
        proj.set_value(String::from("fem"), String::from("is_better_than_great"));

        assert_eq!(proj.get_value("foo"), Some(&String::from("bar4")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_better_than_great")));
    }

    #[test]
    fn remove_value() {
        let mut proj = get_projector(PathBuf::from("/foo/bar"));
        proj.remove_value("foo");
        proj.remove_value("fem");

        assert_eq!(proj.get_value("foo"), Some(&String::from("bar2")));
        assert_eq!(proj.get_value("fem"), Some(&String::from("is_great")));
    }
}