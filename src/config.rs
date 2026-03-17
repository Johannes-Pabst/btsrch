use std::{any::type_name, collections::HashSet};

use serde::{Serialize, de::DeserializeOwned};
use tokio::fs::read_to_string;
use toml::{Table, Value};

pub struct Config {
    data: Table,
    dont_spam_errors: HashSet<String>,
}
impl Config {
    pub async fn load(path: String) -> Self {
        Config {
            data: toml::from_str(read_to_string(path).await.expect("config file \"config.toml\" missing in project root!").as_str()).unwrap(),
            dont_spam_errors: HashSet::new(),
        }
    }
    pub fn get_namespace<T>(&mut self) -> T
    where
        T: DeserializeOwned + Serialize + Default,
    {
        let mut name = type_name::<T>().rsplit("::").next().unwrap();
        if name.ends_with("Config") {
            name = &name[..name.len() - "Config".len()];
        }
        if let Some(a) = self.data.get(name) {
            match a.clone().try_into() {
                Ok(v) => v,
                Err(e) => {
                    let default=T::default();
                    if self.dont_spam_errors.insert(name.to_string()) {
                        println!("{}\nusing default:\n[{name}]\n{}",e.message(), toml::to_string(&default).unwrap());
                    }
                    default
                }
            }
        } else {
            let default=T::default();
            if self.dont_spam_errors.insert(name.to_string()) {
                let mut m=Table::new();
                m.insert(name.to_string(), Value::try_from(&default).unwrap());
                println!("# config file missing entry \"{name}\", using default:\n{}", toml::to_string(&Value::Table(m)).unwrap());
            }
            default
        }
    }
}
