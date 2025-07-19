#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod query_manager;
pub mod test_parser;
pub mod path_parser;
pub mod link_parser;
pub mod custom_commands_parser;
pub mod unit_calc_parser;

use eframe::egui;
use egui::{Align, CentralPanel, FontId, Key, Layout};
use egui::{Frame, TextEdit};
use single_instance::SingleInstance;
use tokio::sync::mpsc;

use crate::custom_commands_parser::CustomCommandsParser;
use crate::link_parser::LinkParser;
use crate::path_parser::PathParser;
use crate::query_manager::{ChangeInstruction, ListEntry, QueryManager};
use crate::unit_calc_parser::main::UnitCalcParser;

struct SearchApp {
    query: String,
    pub layout_receiver: mpsc::Receiver<ChangeInstruction>,
    layout: Vec<ListEntry>,
    pub query_sender: mpsc::Sender<String>,
    selected_id: usize,
}

impl SearchApp {
    fn new(tx: mpsc::Sender<String>, rx: mpsc::Receiver<ChangeInstruction>) -> Self {
        Self {
            query: String::new(),
            layout: Vec::new(),
            query_sender: tx,
            layout_receiver: rx,
            selected_id: usize::MAX,
        }
    }
}

impl eframe::App for SearchApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
            ..egui::Visuals::dark()
        });
        CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                let resp = ui.add(
                    TextEdit::singleline(&mut self.query)
                        .hint_text("Type to search...")
                        .desired_width(f32::INFINITY)
                        .lock_focus(true)
                        .font(FontId::new(24.0, egui::FontFamily::Proportional)),
                );
                resp.request_focus();
                if resp.changed() {
                    let q = self.query.clone();
                    let sender = self.query_sender.clone();
                    tokio::spawn(async move {
                        sender.send(q).await.unwrap();
                    });
                }
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
                                    }
                                }
                            }
                            self.layout
                                .sort_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap());
                            if self.selected_id != usize::MAX {
                                while self.layout[self.selected_id].execute.is_none() {
                                    self.selected_id = (self.selected_id + 1) % self.layout.len();
                                }
                            }
                            for i in 0..self.layout.len() {
                                let l=&mut self.layout[i];
                                let mut brightness=10;
                                if l.execute.is_some(){
                                    brightness=20;
                                    if i==self.selected_id{
                                        brightness=50;
                                    }
                                }
                                Frame::NONE
                                    .fill(egui::Color32::from_rgba_unmultiplied(brightness, brightness, brightness, 200))
                                    .corner_radius(10)
                                    .outer_margin(5)
                                    .inner_margin(5)
                                    .show(ui, |ui| {
                                        (l.layout_fn)(ui);
                                    });
                            }
                        });
                    });
                if ctx.input(|i| i.key_pressed(Key::Escape)) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                    if self.selected_id != usize::MAX {
                        self.selected_id = (self.selected_id + 1) % self.layout.len();
                        while self.layout[self.selected_id].execute.is_none() {
                            self.selected_id = (self.selected_id + 1) % self.layout.len();
                        }
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                    if self.selected_id != usize::MAX {
                        self.selected_id =
                            (self.selected_id - 1 + self.layout.len()) % self.layout.len();
                        while self.layout[self.selected_id].execute.is_none() {
                            self.selected_id =
                                (self.selected_id - 1 + self.layout.len()) % self.layout.len();
                        }
                    }
                }
                if ctx.input(|i| i.key_pressed(Key::Enter)) {
                    if self.selected_id != usize::MAX {
                        (self.layout[self.selected_id].execute.as_mut().unwrap())();
                    }
                }
            });
    }
}

#[tokio::main]
async fn main() {
    let instance = SingleInstance::new("btsrch_unique_app_key_for_the_single_instance_library._It_apparrently_doesn't_matter_what_I_write_here_as_long_as_it's_long_and_noone_else_uses_it,_so_I_wrote_this._Let's_see_if_I_understood_this_correctly.").unwrap();
    if !instance.is_single() {
        return;
    }
    let mut options = eframe::NativeOptions::default();
    options.centered = true;
    options.run_and_return = false;
    options.viewport = egui::ViewportBuilder::default()
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(egui::vec2(500.0, 1000.0))
        .with_always_on_top();
    let (atx, rx) = mpsc::channel::<String>(128);
    let (tx, arx) = mpsc::channel::<ChangeInstruction>(128);
    let app = SearchApp::new(atx, arx);
    let mut mgr = QueryManager::new(rx, tx);
    mgr.add_query_parser::<CustomCommandsParser>();
    mgr.add_query_parser::<LinkParser>();
    mgr.add_query_parser::<PathParser>();
    mgr.add_query_parser::<UnitCalcParser>();
    mgr.start();
    eframe::run_native("BTSRCH", options, Box::new(|_cc| Ok(Box::new(app)))).unwrap();
}
