use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{process::Command, sync::mpsc};

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
            base_priority: 101.0,
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
            use std::process::Stdio;

            let mut t=String::from_utf8(Command::new("bash").arg("-c").arg(format!("compgen -c {}", query)).stdout(Stdio::piped()).spawn().unwrap().wait_with_output().await.unwrap().stdout).unwrap().lines().map(|s| s.to_string()).collect::<Vec<String>>();
            t.dedup();
            for s in t{
                let s_clone=s.clone();
                resopnse
                    .send(ListEntry {
                        layout_fn: Box::new(move |ui| {
                            ui.label(format!("run {} in terminal", &s));
                        }),
                        execute: Some(Box::new(move || {
                            let s_c_c=s_clone.clone();
                            tokio::spawn(async move {
                                run_in_terminal(s_c_c.clone()).await;
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
