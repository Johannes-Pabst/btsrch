use std::{env, sync::Arc};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::{RwLock, mpsc};

use crate::{
    config::Config, query_manager::{ConfigDefault, ListEntry, QueryParser}, search_helper::{SearchConfig, mark_text, search}
};

#[derive(Clone)]
pub struct CustomCommandsParser {
    scripts: Arc<RwLock<Vec<ScriptInfo>>>,
    config: CustomCommandsParserConfig,
    search_config:SearchConfig,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CustomCommandsParserConfig {
    base_priority: f32,
}
impl Default for CustomCommandsParserConfig {
    fn default() -> Self {
        Self { base_priority: 60.0 }
    }
}
#[derive(Clone)]
struct ScriptInfo {
    path: String,
    name: String,
    _stem: String,
    _extension: String,
}
impl ConfigDefault for CustomCommandsParser {
    fn create(config: &mut Config) -> Self {
        let scripts = Arc::new(RwLock::new(Vec::new()));
        let scripts_clone = scripts.clone();
        tokio::spawn(async move {
            let mut s = tokio::fs::read_dir(
                env::current_exe()
                    .unwrap()
                    .ancestors()
                    .nth(3)
                    .unwrap()
                    .join("scripts"),
            )
            .await
            .unwrap();
            let mut s2 = Vec::new();
            while let Some(sc) = s.next_entry().await.unwrap() {
                if sc.file_type().await.unwrap().is_file() {
                    let mut skip = false;
                    #[cfg(target_os = "windows")]
                    {
                        skip |=
                            vec!["sh"].contains(&sc.path().extension().unwrap().to_str().unwrap());
                    }
                    #[cfg(target_os = "linux")]
                    {
                        skip |= vec!["bat", "exe", "ps1", "lnk"]
                            .contains(&sc.path().extension().unwrap().to_str().unwrap());
                    }
                    if !skip {
                        s2.push(ScriptInfo {
                            _extension: sc
                                .path()
                                .extension()
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                            _stem: sc.path().file_stem().unwrap().to_str().unwrap().to_string(),
                            name: sc.path().file_name().unwrap().to_str().unwrap().to_string(),
                            path: sc.path().to_str().unwrap().to_string(),
                        });
                    }
                }
            }
            let mut scripts = scripts_clone.write().await;
            *scripts = s2;
        });
        Self { scripts, config:config.get_namespace(), search_config: config.get_namespace() }
    }
}
#[async_trait]
impl QueryParser for CustomCommandsParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        let mut scripts = self.scripts.read().await;
        while scripts.len() == 0 {
            drop(scripts);
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            scripts = self.scripts.read().await;
        }
        let collect = scripts.iter().map(|s| s.name.clone()).collect();
        for (priority, id, mark) in search(&query, &collect, &self.search_config) {
            let priority=self.config.base_priority+priority;
            let s2 = scripts[id].clone();
            let s3 = scripts[id].clone();
            let value = collect.clone();
            resopnse
                .send(ListEntry {
                    layout_fn: Box::new(move |ui| {
                        mark_text(value[id].clone(), &mark, ui);
                        ui.label(format!("{}", &s2.name));
                    }),
                    execute: Some(Box::new(move || {
                        #[cfg(target_os = "windows")]
                        {
                            open::that_in_background(&s3.path).join().unwrap().unwrap();
                        }
                        #[cfg(target_os = "linux")]
                        {
                            use std::{
                                ffi::OsStr, os::unix::ffi::OsStrExt, path::Path, process::Command,
                            };

                            match Path::new(&s3.path)
                                .extension()
                                .unwrap_or(&OsStr::from_bytes(b""))
                                .to_str()
                                .unwrap()
                            {
                                "" | "sh" => {
                                    let _ = Command::new("bash")
                                        .arg("-c")
                                        .arg(s3.path.clone())
                                        .stdin(std::process::Stdio::null())
                                        .stdout(std::process::Stdio::null())
                                        .stderr(std::process::Stdio::null())
                                        .spawn()
                                        .unwrap();
                                }
                                "url" => {
                                    use std::{fs::File, io::Read};

                                    let mut content = String::new();
                                    File::open(&s3.path)
                                        .unwrap()
                                        .read_to_string(&mut content)
                                        .unwrap();
                                    let arg = content[(content.find("=").unwrap() + 1)..]
                                        .to_string()
                                        .trim()
                                        .to_string();
                                    println!("{}", arg);
                                    let _ = Command::new("xdg-open").arg(arg).spawn().unwrap();
                                }
                                _ => {
                                    open::that_in_background(&s3.path);
                                }
                            }
                        }
                        std::process::exit(0);
                    })),
                    priority,
                })
                .await
                .ok()?;
        }
        None
    }
}
