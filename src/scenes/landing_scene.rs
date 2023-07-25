use super::components::Component;
use super::scene::Scene;
use egui::{Button, CentralPanel, Color32, Context, Direction, Layout, RichText,};
use log::info;

pub struct LandingScene {
    id: u32,
    components: Vec<Box<dyn Component>>,
}

impl Default for LandingScene {
    fn default() -> Self {
        Self {
            id: 0,
            components: vec![],
        }
    }
}

impl Scene for LandingScene {
    fn get_id(&self) -> u32 {
        self.id
    }
    fn render(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        for component in &mut self.components {
            component.render(ctx, _frame);
        }
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(
                        RichText::new("Rust Pantheon Guild")
                            .size(72.0)
                            .color(Color32::from_rgba_premultiplied(192, 192, 192, 120))
                            .background_color(Color32::from_rgb(60, 60, 90))
                            .underline()
                            .strong(),
                    );
                    if ui.add(Button::new("New Game")).clicked() {
                        info!("New Game");
                    }
                    if ui.add(Button::new("Load Game")).clicked() {
                        info!("Load Game");
                    }
                    if ui.add(Button::new("Settings")).clicked() {
                        info!("Load Game");
                    }
                    if ui.add(Button::new("Exit Game")).clicked() {
                        info!("Load Game");
                    }
                })
            })
        });
    }
}
