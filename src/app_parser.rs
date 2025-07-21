use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::{RwLock, mpsc};

use crate::query_manager::{ListEntry, QueryParser};

#[derive(Clone)]
pub struct AppInfo {
    pub name: String,
    pub path: String,
}
#[derive(Clone)]
pub struct AppParser {
    apps: Arc<RwLock<Vec<AppInfo>>>,
}
impl Default for AppParser {
    fn default() -> Self {
        let app_list=Arc::new(RwLock::new(Vec::new()));
        let app_list_clone=app_list.clone();
        tokio::spawn(async move {
            let mut app_list=crawl_folder(r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs".to_string());
            let start_menu_path =  format!(r"\Microsoft\Windows\Start Menu\Programs{}",std::env::var("APPDATA").unwrap());
            app_list.extend(crawl_folder(start_menu_path));
            let mut apps = app_list_clone.write().await;
            apps.extend(app_list);
        });
        Self { apps: app_list }
    }
}
pub fn crawl_folder(path:String)-> Vec<AppInfo> {
    let mut apps = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            apps.push(AppInfo { name: entry.path().file_stem().unwrap().to_str().unwrap().to_string(), path: entry.path().to_str().unwrap().to_string() });
        }
    }
    apps
}
#[async_trait]
impl QueryParser for AppParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) {
        let apps=self.apps.read().await;
        for s in apps.iter() {
            let priority;
            if s.name.to_lowercase().starts_with(&query.to_lowercase()) {
                priority = /* prob = (1/26)^priority */(query.len() as f32) - (apps.len() as f32).log(26.0);
            } else if s.name.contains(&query) {
                priority = (query.len() as f32) - (apps.len() as f32).log(26.0) - ((s.name.len() - query.len()) as f32).log(26.0);
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