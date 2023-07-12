use super::component::Component;

pub struct LeftBarMenu {
  value: f32,
  label: String
}

impl Default for LeftBarMenu {
  fn default() -> Self {
    Self {
      value: 3.0,
      label: "Hello World!".to_owned(),
    }
  }
}

impl Component for LeftBarMenu {
  fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let Self { label, value } = self;

    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.heading("Side Panel");

        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(label);
        });

        ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            *value += 1.0;
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("powered by ");
                ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                ui.label(" and ");
                ui.hyperlink_to(
                    "eframe",
                    "https://github.com/emilk/egui/tree/master/crates/eframe",
                );
                ui.label(".");
            });
        });
    });
  }
}