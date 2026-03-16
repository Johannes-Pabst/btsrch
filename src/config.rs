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
        T: DeserializeOwned + Default,
    {
        let mut name = type_name::<T>().rsplit("::").next().unwrap();
        if name.ends_with("Config"){
            name=&name[..name.len()-"Config".len()];
        }
        if let Some(a) = self.data.get(name) {
            match a.clone().try_into() {
                Ok(v)=>v,
                Err(e)=>{
                    println!("{:?}", e);
                    T::default()
                }
            }
        }else{
            println!("config file missing entry \"{}\"", name);
            T::default()
        }
    }
}
