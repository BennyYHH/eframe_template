use super::component::Component;
use super::window_component::WindowComponent;
use egui::{ScrollArea, SelectableLabel};
use std::collections::BTreeSet;

pub struct LeftBarMenu {
    value: f32,
    label: String,
    open_state: BTreeSet<String>,

    sub_window: Vec<Box<dyn WindowComponent>>,
}

impl Default for LeftBarMenu {
    fn default() -> Self {
        Self {
            value: 3.0,
            label: "Hello World!".to_owned(),
            open_state: BTreeSet::new(),

            sub_window: Vec::new(),
        }
    }
}

impl Component for LeftBarMenu {
    fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self {
        //     label,
        //     value,
        //     open_state,
        // } = self;

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading("Menu Panel");
                });

                ScrollArea::vertical().show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.separator();

                        append_toggle_button(
                            ui,
                            &mut self.open_state,
                            "Button1",
                            "Button1",
                        );

                        append_toggle_button(
                            ui,
                            &mut self.open_state,
                            "Button2",
                            "Button2",
                        );
                    });
                });
            });
    }
}

fn append_toggle_button(
    ui: &mut egui::Ui,
    open_state: &mut BTreeSet<String>,
    text: &str,
    key: & str,
) {
    let mut is_open = open_state.contains(key);
    if ui
        .add_sized([150.0, 40.0], SelectableLabel::new(is_open, text))
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
