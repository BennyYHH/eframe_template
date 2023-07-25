use super::{component::Component, window_component::WindowComponent};
use egui::{RichText, ScrollArea, SelectableLabel};
use std::collections::BTreeSet;

pub struct LeftBarMenu {
    open_state: BTreeSet<String>,

    sub_windows: Vec<Box<dyn WindowComponent>>,
    map_windows: Vec<Box<dyn WindowComponent>>,
}

impl Default for LeftBarMenu {
    fn default() -> Self {
        Self {
            open_state: BTreeSet::new(),

            sub_windows: vec![
                Box::new(super::item_window::ItemWindow::default()),
                Box::new(super::equipment_window::EquipmentWindow::default()),
                Box::new(super::skill_window::SkillWindow::default()),
                Box::new(super::party_window::PartyWindow::default()),
                Box::new(super::save_load_window::SaveLoadWindow::default()),
                Box::new(super::setting_window::SettingWindow::default()),
            ],

            map_windows: vec![
                Box::new(super::global_map_window::GlobalMapWindow::default()),
                Box::new(super::local_map_window::LocalMapWindow::default()),
            ],
        }
    }
}

impl Component for LeftBarMenu {
    fn render(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(140.0)
            .show(_ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Main Menu");
                });

                ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.separator();

                        for window in self.sub_windows.iter_mut() {
                            let key = window.get_id();
                            let text = window.get_button_name();
                            append_toggle_button(ui, &mut self.open_state, &text, &key, [140.0, 40.0]);
                        }
                    });
                });

                egui::TopBottomPanel::bottom("map_panel")
                    .resizable(false)
                    .show_inside(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading("Map");
                        });
                        ui.horizontal_wrapped(|ui| {
                            for window in self.map_windows.iter_mut() {
                                let key = window.get_id();
                                let text = window.get_button_name();
                                append_toggle_button(ui, &mut self.open_state, &text, &key, [45.0, 40.0]);
                            }
                        });
                    });
            });
        egui::CentralPanel::default().show(_ctx, |_: &mut egui::Ui| {
            self.show_windows(_ctx, _frame);
            self.show_map_windows(_ctx, _frame);
        });
    }
}

impl LeftBarMenu {
    pub fn show_windows(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            sub_windows,
            open_state,
            ..
        } = self;
        for sub_window in sub_windows {
            let mut is_open = open_state.contains(sub_window.get_id());
            WindowComponent::render(sub_window.as_mut(), ctx, _frame, &mut is_open);
            set_open(open_state, sub_window.get_id(), is_open);
        }
    }

    pub fn show_map_windows(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            map_windows,
            open_state,
            ..
        } = self;
        for map_window in map_windows {
            let mut is_open = open_state.contains(map_window.get_id());
            WindowComponent::render(map_window.as_mut(), ctx, _frame, &mut is_open);
            set_open(open_state, map_window.get_id(), is_open);
        }
    }
}

fn append_toggle_button(
    ui: &mut egui::Ui,
    open_state: &mut BTreeSet<String>,
    text: &str,
    key: &str,
    size: [f32; 2],
) {
    let mut is_open = open_state.contains(key);
    if ui
        .add_sized(
            size,
            SelectableLabel::new(is_open, RichText::new(text).size(16.0)),
        )
        .clicked()
    {
        is_open = !is_open;
        set_open(open_state, &key, is_open);
    };
}

fn set_open(open_state: &mut BTreeSet<String>, key: &str, is_open: bool) {
    if is_open {
        if !open_state.contains(key) {
            open_state.insert(key.to_owned());
        }
    } else {
        open_state.remove(key);
    }
}
