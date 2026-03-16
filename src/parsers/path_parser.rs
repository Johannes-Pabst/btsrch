use std::path::Path;

use async_trait::async_trait;
use serde::Deserialize;
use tokio::sync::mpsc;

use crate::query_manager::{ConfigDefault, ListEntry, QueryParser};

#[derive(Clone)]
pub struct PathParser {
    config: PathParserConfig,
}
#[derive(Clone, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PathParserConfig {
    base_priority: f32,
}
impl Default for PathParserConfig {
    fn default() -> Self {
        Self { base_priority: 100.0 }
    }
}
impl ConfigDefault for PathParser {
    fn create(config:&mut crate::config::Config)->Self {
        Self { config: config.get_namespace() }
    }
}
#[async_trait]
impl QueryParser for PathParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        if Path::new(&query).exists() {
            let q2 = query.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        ui.label(format!("open {}", &query));
                    }),
                    execute: Some(Box::new(move || {
                        #[cfg(target_os = "linux")]
                        open::that_in_background(&q2);
                        #[cfg(target_os = "windows")]
                        open::that_in_background(&q2).join().unwrap().unwrap();
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
