use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::{process::Command, sync::mpsc};

use crate::query_manager::{ConfigDefault, ListEntry, QueryParser};

#[derive(Clone)]
pub struct LinkParser {
    config: LinkParserConfig,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct LinkParserConfig {
    base_priority: f32,
}
impl Default for LinkParserConfig {
    fn default() -> Self {
        Self {
            base_priority: 101.0,
        }
    }
}
impl ConfigDefault for LinkParser {
    fn create(config: &mut crate::config::Config) -> Self {
        Self {
            config: config.get_namespace(),
        }
    }
}
#[async_trait]
impl QueryParser for LinkParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        let mut t=String::from_utf8(Command::new("compgen").arg("-p").arg(query).spawn().unwrap().wait_with_output().await.unwrap().stdout).unwrap().lines().collect::<Vec<String>>();
        // t.dedup();
        for s in t{
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        ui.label(format!("open {} in the browser", &q2));
                    }),
                    execute: Some(Box::new(move || {
                        #[cfg(target_os = "windows")]
                        open::that_in_background(&final_link)
                            .join()
                            .unwrap()
                            .unwrap();
                        #[cfg(target_os = "linux")]
                        open::that_in_background(&final_link);
                        std::process::exit(0);
                    })),
                    priority: self.config.base_priority,
                })
                .await
                .ok()?;
        }
        None
    }
}
