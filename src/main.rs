#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod config;
pub mod parsers;
pub mod query_manager;
pub mod search_helper;

use std::sync::Arc;

use eframe::egui::{self, Color32, CornerRadius, Margin, Stroke};
use egui::{Align, CentralPanel, FontId, Key, Layout, Modifiers, Shadow};
use egui::{Frame, TextEdit};
use existing_instance::Endpoint;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::Config;
use crate::parsers::app_parser::AppParser;
use crate::parsers::custom_commands_parser::CustomCommandsParser;
use crate::parsers::link_parser::LinkParser;
use crate::parsers::path_parser::PathParser;
use crate::parsers::unicode_parser::UnicodeParser;
use crate::parsers::unit_calc_parser::main::UnitCalcParser;
use crate::query_manager::{ChangeInstruction, ListEntry, QueryManager};

struct SearchApp {
    query: String,
    pub layout_receiver: mpsc::Receiver<ChangeInstruction>,
    layout: Vec<ListEntry>,
    pub query_sender: mpsc::Sender<String>,
    selected_id: usize,
    scroll_todo: bool,
    had_focus: bool,
    last_input: String,
    first: bool,
    config: UIConfig,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct UIConfig {
    textbox_frame: FrameConfig,
    outer_frame: FrameConfig,
    non_selected_result_frame:FrameConfig,
    selected_result_frame:FrameConfig,
}
impl Default for UIConfig {
    fn default() -> Self {
        Self {
            textbox_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
            outer_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
            non_selected_result_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
            selected_result_frame: FrameConfig {
                inner_margin: MarginConfig {
                    left: 0,
                    right: 0,
                    top: 0,
                    bottom: 0,
                },
                fill: RgbaConfig {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
                stroke: StrokeConfig {
                    width: 0.0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                },
                corner_radius: CornerRadiusConfig::One(0),
                outer_margin: MarginConfig {
                    bottom: 0,
                    left: 0,
                    right: 0,
                    top: 0,
                },
                shadow: ShadowConfig {
                    blur: 0,
                    color: RgbaConfig {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 0,
                    },
                    offset_x: 0,
                    offset_y: 0,
                    spread: 0,
                },
            },
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RgbaConfig {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}
impl Default for RgbaConfig {
    fn default() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ShadowConfig {
    pub offset_x: i8,
    pub offset_y: i8,
    pub blur: u8,
    pub spread: u8,
    pub color: RgbaConfig,
}
impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            offset_x: 0,
            offset_y: 0,
            blur: 0,
            spread: 0,
            color: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FrameConfig {
    pub inner_margin: MarginConfig,
    pub fill: RgbaConfig,
    pub stroke: StrokeConfig,
    pub corner_radius: CornerRadiusConfig,
    pub outer_margin: MarginConfig,
    pub shadow: ShadowConfig,
}
impl Default for FrameConfig {
    fn default() -> Self {
        Self {
            inner_margin: Default::default(),
            fill: Default::default(),
            stroke: Default::default(),
            corner_radius: Default::default(),
            outer_margin: Default::default(),
            shadow: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MarginConfig {
    pub left: i8,
    pub right: i8,
    pub top: i8,
    pub bottom: i8,
}
impl Default for MarginConfig {
    fn default() -> Self {
        Self {
            left: Default::default(),
            right: Default::default(),
            top: Default::default(),
            bottom: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct StrokeConfig {
    pub width: f32,
    pub color: RgbaConfig,
}
impl Default for StrokeConfig {
    fn default() -> Self {
        Self {
            width: Default::default(),
            color: Default::default(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CornerRadiusConfig {
    One(u8),
    All(CornerRadiusDetailedConfig),
}
impl Default for CornerRadiusConfig {
    fn default() -> Self {
        Self::One(0)
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CornerRadiusDetailedConfig {
    pub nw: u8,
    pub ne: u8,
    pub sw: u8,
    pub se: u8,
}
impl Default for CornerRadiusDetailedConfig {
    fn default() -> Self {
        Self {
            nw: Default::default(),
            ne: Default::default(),
            sw: Default::default(),
            se: Default::default(),
        }
    }
}
impl From<FrameConfig> for Frame{
    fn from(value: FrameConfig) -> Self {
        Frame { inner_margin: value.inner_margin.into(), fill: value.fill.into(), stroke: value.stroke.into(), corner_radius: value.corner_radius.into(), outer_margin: value.outer_margin.into(), shadow: value.shadow.into() }
    }
}
impl From<MarginConfig> for Margin{
    fn from(value: MarginConfig) -> Self {
        Margin { left:value.left, right:value.right, top:value.top, bottom:value.bottom }
    }
}
impl From<RgbaConfig> for Color32{
    fn from(value: RgbaConfig) -> Self {
        Color32::from_rgba_unmultiplied(value.r, value.g, value.b, value.a)
    }
}
impl From<StrokeConfig> for Stroke{
    fn from(value: StrokeConfig) -> Self {
        Stroke { width: value.width, color: value.color.into() }
    }
}
impl From<CornerRadiusConfig> for CornerRadius{
    fn from(value: CornerRadiusConfig) -> Self {
        match value{
            CornerRadiusConfig::All(c)=>CornerRadius { nw: c.nw, ne: c.ne, sw: c.sw, se: c.se },
            CornerRadiusConfig::One(x)=>CornerRadius::same(x),
        }
    }
}
impl From<ShadowConfig> for Shadow{
    fn from(value: ShadowConfig) -> Self {
        Shadow { offset: [value.offset_x, value.offset_y], blur: value.blur, spread: value.spread, color: value.color.into() }
    }
}
impl SearchApp {
    fn new(
        tx: mpsc::Sender<String>,
        rx: mpsc::Receiver<ChangeInstruction>,
        config: &mut Config,
    ) -> Self {
        Self {
            query: String::new(),
            layout: Vec::new(),
            query_sender: tx,
            layout_receiver: rx,
            selected_id: usize::MAX,
            had_focus: false,
            scroll_todo: false,
            last_input: String::new(),
            first: true,
            config: config.get_namespace(),
        }
    }
}

impl eframe::App for SearchApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        image_extras::register();
        egui_extras::install_image_loaders(ctx);
        CentralPanel::default()
            .frame(self.config.outer_frame.clone().into())
            .show(ctx, |ui| {
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::ArrowDown)) {
                    if self.selected_id != usize::MAX {
                        self.selected_id = (self.selected_id + 1) % self.layout.len();
                        while self.layout[self.selected_id].execute.is_none() {
                            self.selected_id = (self.selected_id + 1) % self.layout.len();
                        }
                        self.scroll_todo = true;
                    }
                }
                if ctx.input_mut(|i| i.consume_key(Modifiers::NONE, Key::ArrowUp)) {
                    if self.selected_id != usize::MAX {
                        self.selected_id =
                            (self.selected_id - 1 + self.layout.len()) % self.layout.len();
                        while self.layout[self.selected_id].execute.is_none() {
                            self.selected_id =
                                (self.selected_id - 1 + self.layout.len()) % self.layout.len();
                        }
                        self.scroll_todo = true;
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    if self.selected_id != usize::MAX {
                        (self.layout[self.selected_id].execute.as_mut().unwrap())();
                    }
                }
                if ctx.input(|i| i.focused) {
                    self.had_focus = true;
                }
                if ctx.input(|i| !i.focused) {
                    if self.had_focus {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                let f:Frame=self.config.textbox_frame.clone().into();
                f
                    .show(ui, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            let resp = ui.add(
                                TextEdit::singleline(&mut self.query)
                                    // .hint_text("Type to search...")
                                    .desired_width(f32::INFINITY)
                                    .lock_focus(true)
                                    .font(FontId::new(24.0, egui::FontFamily::Proportional))
                                    .frame(false),
                            );
                            resp.request_focus();
                            if resp.changed() {
                                let q = self.query.clone();
                                if self.last_input != q {
                                    let sender = self.query_sender.clone();
                                    self.last_input = q.clone();
                                    tokio::spawn(async move {
                                        sender.send(q).await.unwrap();
                                    });
                                }
                            }
                        });
                    });
                egui::ScrollArea::vertical()
                    .scroll_bar_visibility(
                        egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded,
                    )
                    .show(ui, |ui| {
                        ui.with_layout(Layout::top_down(Align::Center), |ui| {
                            while let Ok(l) = self.layout_receiver.try_recv() {
                                match l {
                                    ChangeInstruction::Add(la) => {
                                        if self.selected_id == usize::MAX && la.execute.is_some() {
                                            self.selected_id = self.layout.len();
                                        }
                                        self.layout.push(la);
                                    }
                                    ChangeInstruction::Empty => {
                                        self.layout.clear();
                                        self.selected_id = usize::MAX;
                                        self.scroll_todo = false;
                                    }
                                }
                            }
                            self.layout
                                .sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
                            if self.selected_id != usize::MAX {
                                while self.layout[self.selected_id].execute.is_none() {
                                    self.selected_id = (self.selected_id + 1) % self.layout.len();
                                }
                            }
                            for i in 0..self.layout.len() {
                                let l = &mut self.layout[i];
                                let frame:Frame=if i==self.selected_id{
                                    self.config.selected_result_frame.clone()
                                }else{
                                    self.config.non_selected_result_frame.clone()
                                }.into();
                                let frame=frame.show(ui, |ui| {
                                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                            (l.layout_fn)(ui);
                                        });
                                    });
                                if self.scroll_todo && self.selected_id == i {
                                    frame.response.scroll_to_me(None);
                                    self.scroll_todo = false;
                                }
                            }
                        });
                    });
            });
        if self.first {
            ctx.set_visuals(egui::Visuals {
                window_fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
                ..egui::Visuals::dark()
            });
            let mut style = (*ctx.style()).clone();
            style.visuals.override_text_color = Some(egui::Color32::WHITE);
            ctx.set_style(style);

            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "unifont".to_owned(),
                Arc::new(egui::FontData::from_static(include_bytes!(
                    r"../UnifontExMono.ttf"
                ))),
            );
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "unifont".to_owned());
            ctx.set_fonts(fonts);
            self.first = false;
        }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let mut options = eframe::NativeOptions::default();
    options.run_and_return = false;
    let endpoint = existing_instance::establish_endpoint("btsrch_short_unique_key", true).unwrap();
    if let Endpoint::Existing(_) = endpoint {
        println!("already open...");
        std::process::exit(0);
    }
    #[cfg(target_os = "windows")]
    {
        options.centered = true;
        options.viewport = egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_inner_size(egui::vec2(500.0, 1000.0))
            .with_always_on_top()
            .with_active();
    }
    #[cfg(target_os = "linux")]
    {
        use x11rb::{connection::Connection, protocol::randr::ConnectionExt};

        let mut width: f32 = 500.0;
        let mut height: f32 = 1000.0;
        options.centered = true;
        options.viewport = egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_active(true);
        if let Ok((conn, screen_num)) = x11rb::connect(None) {
            let roots = &conn.setup().roots[screen_num];
            let screen = roots;
            let primary_id = conn
                .randr_get_output_primary(screen.root)
                .unwrap()
                .reply()
                .unwrap()
                .output;
            if let Ok(temp) = conn.randr_get_output_info(primary_id, 0).unwrap().reply() {
                let primary_crtc = temp.crtc;
                let primary_info = conn
                    .randr_get_crtc_info(primary_crtc, 0)
                    .unwrap()
                    .reply()
                    .unwrap();
                width = primary_info.width as f32 * 0.3;
                height = primary_info.height as f32 * 0.8;
                let x = primary_info.x + ((primary_info.width / 2) as i16) - (width as i16) / 2;
                let y = primary_info.y + ((primary_info.height / 2) as i16) - (height as i16) / 2;
                options.viewport = options.viewport.with_position((x as f32, y as f32));
                options.centered = false;
            }
        }
        options.viewport = options.viewport.with_inner_size(egui::vec2(width, height));
    }
    let (atx, rx) = mpsc::channel::<String>(128);
    let (tx, arx) = mpsc::channel::<ChangeInstruction>(128);
    let mut config = Config::load(
        std::env::current_exe()
            .unwrap()
            .ancestors()
            .nth(3)
            .unwrap()
            .join("config.toml")
            .to_str()
            .unwrap()
            .to_string(),
    )
    .await;
    let mut mgr = QueryManager::new(rx, tx).await;
    let app = SearchApp::new(atx, arx, &mut config);
    let a = tokio::task::spawn_blocking(|| async move {
        mgr.add_query_parser_config::<CustomCommandsParser>(&mut config);
        mgr.add_query_parser_config::<LinkParser>(&mut config);
        mgr.add_query_parser_config::<PathParser>(&mut config);
        mgr.add_query_parser_config::<UnitCalcParser>(&mut config);
        mgr.add_query_parser_config::<AppParser>(&mut config);
        mgr.add_query_parser_config::<UnicodeParser>(&mut config);
        mgr.start().await.unwrap();
    });
    tokio::spawn(async move {
        a.await.unwrap().await;
    });
    eframe::run_native("BTSRCH", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}
