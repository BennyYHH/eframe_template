use super::window_component::WindowComponent;
use crate::scenes::components::Component;

pub struct LocalMapWindow {
    id: String,
    button_name: String,
    window_name: String,

    pub logarithmic: bool,
    pub clamp_to_range: bool,
    pub smart_aim: bool,
}

impl Default for LocalMapWindow {
    fn default() -> Self {
        Self {
            id: "local_map_window".to_owned(),
            button_name: "Local".to_owned(),
            window_name: "Global Map".to_owned(),

            logarithmic: false,
            clamp_to_range: false,
            smart_aim: false,
        }
    }
}

impl WindowComponent for LocalMapWindow {
    fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, open: &mut bool) {
        egui::Window::new(self.get_window_name())
            .open(open)
            .resizable(false)
            .show(ctx, |ui| {
              ui.checkbox(&mut self.logarithmic, "Logarithmic");
              ui.label("Logarithmic sliders are great for when you want to span a huge range, i.e. from zero to a million.");
              ui.label("Logarithmic sliders can include infinity and zero.");
              ui.add_space(8.0);
      
              ui.checkbox(&mut self.clamp_to_range, "Clamp to range");
              ui.label("If true, the slider will clamp incoming and outgoing values to the given range.");
              ui.label("If false, the slider can shows values outside its range, and you can manually enter values outside the range.");
              ui.add_space(8.0);
      
              ui.checkbox(&mut self.smart_aim, "Smart Aim");
              ui.label("Smart Aim will guide you towards round values when you drag the slider so you you are more likely to hit 250 than 247.23");
              ui.add_space(8.0);
            });
    }

    fn get_id(&self) -> &str {
        self.id.as_str()
    }

    fn get_button_name(&self) -> &str {
        self.button_name.as_str()
    }

    fn get_window_name(&self) -> &str {
        self.window_name.as_str()
    }
}

impl Component for LocalMapWindow {
    fn render(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
