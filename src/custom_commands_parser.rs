use std::env;

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::query_manager::{ListEntry, QueryParser};

#[derive(Clone)]
pub struct CustomCommandsParser {
    scripts: Vec<ScriptInfo>,
}
#[derive(Clone)]
struct ScriptInfo {
    path: String,
    name: String,
    _stem: String,
    _extension: String,
}
impl Default for CustomCommandsParser {
    fn default() -> Self {
        let scripts = std::fs::read_dir(
            env::current_exe()
                .unwrap()
                .ancestors()
                .nth(3)
                .unwrap()
                .join("scripts"),
        )
        .unwrap()
        .map(|c| {
            let path = c.unwrap().path();
            ScriptInfo {
                    path: path.to_str().unwrap().to_string(),
                    _extension: path
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    name: path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    _stem: path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                }
        })
        .collect::<Vec<ScriptInfo>>();
        Self { scripts }
    }
}
#[async_trait]
impl QueryParser for CustomCommandsParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) {
        for s in self.scripts.iter() {
            let priority;
            if s.name.to_lowercase().starts_with(&query.to_lowercase()) {
                priority = /* prob = (1/26)^priority */(query.len() as f32) - (self.scripts.len() as f32).log(26.0);
            } else if s.name.contains(&query) {
                priority = (query.len() as f32) - (self.scripts.len() as f32).log(26.0) - ((s.name.len() - query.len()) as f32).log(26.0);
            } else {
                continue;
            }
            let s2 = s.clone();
            let s3 = s.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        ui.label(format!("{}", &s2.name));
                    }),
                    execute: Some(Box::new(move || {
                        open::that_in_background(&s3.path).join().unwrap().unwrap();
                        std::process::exit(0);
                    })),
                    priority,
                })
                .await
                .unwrap();
        }
    }
}
