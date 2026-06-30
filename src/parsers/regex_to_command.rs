use std::process::exit;

use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tokio::{process::Command, sync::mpsc};

use crate::query_manager::{ConfigDefault, ListEntry, QueryParser};

#[derive(Clone)]
pub struct RegexToCommandParser {
    elements: Vec<RegexToCommandElementParsed>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RegexToCommandParserConfig {
    elements: Vec<RegexToCommandElement>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RegexToCommandElement {
    base_priority: f32,
    match_regex: String,
    command_template: String,
}
#[derive(Clone)]
pub struct RegexToCommandElementParsed {
    base_priority: f32,
    match_regex: regex::Regex,
    command_template: String,
}
impl Default for RegexToCommandParserConfig {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}
impl Default for RegexToCommandElement {
    fn default() -> Self {
        Self {
            base_priority: 100.0,
            command_template: String::new(),
            match_regex: String::new(),
        }
    }
}
impl ConfigDefault for RegexToCommandParser {
    fn create(config: &mut crate::config::Config) -> Self {
        let config: RegexToCommandParserConfig = config.get_namespace();
        Self {
            elements: config
                .elements
                .iter()
                .map(|x| RegexToCommandElementParsed {
                    base_priority: x.base_priority,
                    match_regex: Regex::new(&x.match_regex).unwrap(),
                    command_template: x.command_template.clone(),
                })
                .collect(),
        }
    }
}
enum HelpHowDoINameThis {
    Text(String),
    Replace(String),
}
#[async_trait]
impl QueryParser for RegexToCommandParser {
    async fn parse(&self, query: String, resopnse: mpsc::Sender<ListEntry>) -> Option<()> {
        for e in self.elements.iter() {
            let captures = e.match_regex.captures(&query);
            if let Some(captures) = captures {
                let mut command = vec![HelpHowDoINameThis::Text(e.command_template.clone())];
                for c in e.match_regex.capture_names() {
                    if let Some(c) = c {
                        let value = captures.name(c).unwrap();
                        command = command
                            .drain(..)
                            .flat_map(|x| match x {
                                HelpHowDoINameThis::Text(t) => {
                                    let (mut v, u) = t.match_indices(c).fold(
                                        (Vec::new(), 0),
                                        |(mut a, st), (u, s)| {
                                            if u > st {
                                                a.push(HelpHowDoINameThis::Text(
                                                    t[st..u].to_string(),
                                                ));
                                            }
                                            a.push(HelpHowDoINameThis::Replace(
                                                value.as_str().to_string(),
                                            ));
                                            (a, u + s.len())
                                        },
                                    );
                                    if u < t.len()-1 {
                                        v.push(HelpHowDoINameThis::Text(t[u..].to_string()));
                                    }
                                    v
                                }
                                a => vec![a],
                            })
                            .collect();
                    }
                }
                let command = command
                    .drain(..)
                    .map(|x| match x {
                        HelpHowDoINameThis::Replace(s) => s,
                        HelpHowDoINameThis::Text(s) => s,
                    })
                    .collect::<String>();
                let command_clone = command.clone();
                resopnse
                    .send(ListEntry {
                        layout_fn: Box::new(move |ui| {
                            ui.label(format!("run command \"{}\"", &command));
                        }),
                        execute: Some(Box::new(move || {
                            Command::new("bash")
                                .args(&["-c", &command_clone])
                                .spawn()
                                .unwrap();
                            exit(0);
                        })),
                        priority: e.base_priority,
                    })
                    .await
                    .ok()?;
            }
        }
        None
    }
}