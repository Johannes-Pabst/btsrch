use std::any::type_name;

use serde::de::DeserializeOwned;
use tokio::fs::read_to_string;
use toml::Table;

pub struct Config {
    data: Table,
}
impl Config {
    pub async fn load(path: String) -> Self {
        Config {
            data: toml::from_str(read_to_string(path).await.unwrap().as_str()).unwrap(),
        }
    }
    pub fn get_namespace<T>(&self) -> T
    where
        T: ConfigType,
    {
        self.data
            .get(type_name::<T>().rsplit("::").next().unwrap())
            .unwrap()
            .clone()
            .try_into()
            .unwrap()
    }
}
pub trait ConfigType: DeserializeOwned {}
