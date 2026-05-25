use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
    process::Command,
    sync::{RwLock, mpsc},
};

use crate::{
    query_manager::{ConfigDefault, ListEntry, QueryParser},
    search_helper::SearchConfig,
};

#[derive(Clone)]
pub struct ClipboardParser {
    config: ClipboardParserConfig,
    search_config: SearchConfig,
    data: Arc<RwLock<Option<Vec<ClipboardEntry>>>>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ClipboardParserConfig {
    new_priority: f32,
    old_priority: f32,
    max_searchable_len: u64,
}
impl Default for ClipboardParserConfig {
    fn default() -> Self {
        Self {
            new_priority: 45.0,
            old_priority: 45.0,
            max_searchable_len: 1000,
        }
    }
}
impl ConfigDefault for ClipboardParser {
    fn create(config: &mut crate::config::Config) -> Self {
        let data = Arc::new(RwLock::new(None));
        let data_clone = data.clone();
        let clipboard_config: ClipboardParserConfig = config.get_namespace();
        tokio::spawn(async move {
            let d = cliphist_parsed(clipboard_config.max_searchable_len)
                .await
                .unwrap();
            *data_clone.write().await = Some(d);
        });
        Self {
            config: clipboard_config,
            search_config: config.get_namespace(),
            data,
        }
    }
}
#[async_trait]
impl QueryParser for ClipboardParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        #[cfg(target_os = "linux")]
        {
            use crate::search_helper::search;

            let mut data: Option<Vec<ClipboardEntry>> = None;
            while let None = data {
                data = self.data.read().await.clone();
            }
            let data = data.unwrap();
            let data_searchable = data
                .iter()
                .enumerate()
                .map(|(i, x)| format!("📋{}: {}", i, x.preview.clone()))
                .collect();
            for (add_priority, i, mark) in search(&query, &data_searchable, &self.search_config) {
                let value = data_searchable[i].clone();
                let data_clone = data[i].clone();
                resopnse
                    .send(ListEntry {
                        layout_fn: Box::new(move |ui| {
                            use crate::search_helper::mark_text;

                            mark_text(value.clone(), &mark, ui);
                        }),
                        execute: Some(Box::new(move || {
                            use tokio::task::spawn_blocking;

                            let data_clone_clone = data_clone.clone();
                            tokio::spawn(async move {
                                spawn_blocking(async move || {
                                    use std::process::Stdio;

                                    let mut wl_copy = std::process::Command::new("wl-copy")
                                        .stdin(Stdio::piped())
                                        .spawn()
                                        .unwrap();
                                    let _cliphist = std::process::Command::new("cliphist")
                                        .arg("decode")
                                        .arg(data_clone_clone.id.to_string())
                                        .stdout(wl_copy.stdin.take().unwrap())
                                        .spawn()
                                        .unwrap();
                                    wl_copy.wait().unwrap();
                                    std::process::exit(0);
                                })
                                .await
                                .unwrap()
                                .await;
                            });
                        })),
                        priority: self.config.new_priority
                            + (self.config.old_priority - self.config.new_priority) * i as f32
                                / (data.len() - 1) as f32
                            + add_priority,
                    })
                    .await
                    .ok()?;
            }
        }
        None
    }
}
#[cfg(target_os = "linux")]
pub async fn cliphist_raw(preview_width: u64) -> Result<String, String> {
    use std::process::Stdio;

    Ok(String::from_utf8_lossy(
        &Command::new("cliphist")
            .arg("-preview-width")
            .arg(preview_width.to_string())
            .arg("list")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{:?}", e))?
            .wait_with_output()
            .await
            .map_err(|e| format!("{:?}", e))?
            .stdout,
    ).to_string())
}
#[cfg(target_os = "linux")]
pub async fn cliphist_split(preview_width: u64) -> Result<Vec<(u64, String)>, String> {
    let r = cliphist_raw(preview_width).await?;
    Ok(r.lines()
        .map(|e| {
            let (a, b) = e.split_once('\t').unwrap();
            (a.parse::<u64>().unwrap(), b.to_string())
        })
        .collect::<Vec<_>>())
}
#[cfg(target_os = "linux")]
pub async fn cliphist_parsed(preview_width: u64) -> Result<Vec<ClipboardEntry>, String> {
    let mut long = cliphist_split(preview_width).await?;
    let short = cliphist_split(0).await?;
    Ok(long
        .drain(..)
        .map(|x| {
            let image = short
                .binary_search_by_key(&(u64::MAX - x.0), |y| u64::MAX - y.0)
                .map(|id| {
                    let s = &short[id];
                    (s.1 != "…".to_string()).then(|| {
                        let mut args = s.1.split_whitespace();
                        args.next().unwrap();
                        args.next().unwrap();
                        args.next().unwrap();
                        let size_mult = args.next().unwrap().parse::<u64>().unwrap();
                        let unit_mult = match args.next().unwrap() {
                            "B" => 1,
                            "KiB" => 1024,
                            "MiB" => 1024 * 1024,
                            "GiB" => 1024 * 1024 * 1024,
                            u => panic!("unknown unit: {}", u),
                        };
                        let format = args.next().unwrap();
                        let size = args.next().unwrap();
                        let (width, height) = size.split_once('x').unwrap();
                        ClipboardImageData {
                            bytes: size_mult * unit_mult,
                            format: format.to_string(),
                            height: height.parse().unwrap(),
                            width: width.parse().unwrap(),
                        }
                    })
                })
                .ok()
                .flatten();
            ClipboardEntry {
                id: x.0,
                preview: x.1,
                decode_priority: (image.is_some() as u8 as f32) * 10.0,
                decoded: Arc::new(RwLock::new(None)),
                image,
            }
        })
        .collect::<Vec<_>>())
}
#[cfg(target_os = "linux")]
#[derive(Clone)]
pub struct ClipboardEntry {
    pub id: u64,
    pub preview: String,
    pub image: Option<ClipboardImageData>,
    pub decode_priority: f32,
    pub decoded: Arc<RwLock<Option<DecodeExtraData>>>,
}
#[cfg(target_os = "linux")]
pub struct DecodeExtraData {
    pub raw: Option<Vec<u8>>,
    pub raw_string: Option<String>,
    pub cached_image_preview_path: Option<String>,
}
#[cfg(target_os = "linux")]
#[derive(Clone)]
pub struct ClipboardImageData {
    pub bytes: u64,
    pub format: String,
    pub width: u64,
    pub height: u64,
}
