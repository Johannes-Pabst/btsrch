use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[cfg(target_os = "linux")]
use crate::os_utils::run_in_terminal;
use crate::query_manager::{ConfigDefault, ListEntry, QueryParser};

#[derive(Clone)]
pub struct CommandParser {
    config: CommandParserConfig,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CommandParserConfig {
    base_priority: f32,
}
impl Default for CommandParserConfig {
    fn default() -> Self {
        Self {
            base_priority: 70.0,
        }
    }
}
impl ConfigDefault for CommandParser {
    fn create(config: &mut crate::config::Config) -> Self {
        Self {
            config: config.get_namespace(),
        }
    }
}
#[async_trait]
impl QueryParser for CommandParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        #[cfg(target_os = "linux")]
        {
            use crate::os_utils::exists_command;

            if exists_command(&query).await{
                let s_clone=query.clone();
                resopnse
                    .send(ListEntry {
                        layout_fn: Box::new(move |ui| {
                            ui.label(format!("run {} in terminal", &query));
                        }),
                        execute: Some(Box::new(move || {
                            let s_c_c=s_clone.clone();
                            tokio::spawn(async move {
                                run_in_terminal(format!("bash -c \"{}; exec bash\"",&s_c_c), true).await;
                                std::process::exit(0);
                            });
                        })),
                        priority: self.config.base_priority,
                    })
                    .await
                    .ok()?;
            }
        }
        None
    }
}
