use crate::scenes::components::Component;

use super::window_component::WindowComponent;

pub struct SettingWindow {
    id: String,
    button_name: String,
    window_name: String,
}

impl Default for SettingWindow {
    fn default() -> Self {
        Self {
            id: "setting_window".to_owned(),
            button_name: "Setting".to_owned(),
            window_name: "Setting Window".to_owned(),
        }
    }
}

impl WindowComponent for SettingWindow {
  fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, open: &mut bool) {
    egui::Window::new(self.get_window_name())
        .open(open)
        .resizable(false)
        .show(ctx, |ui| {
          ui.label("Logarithmic sliders are great for when you want to span a huge range, i.e. from zero to a million.");
          ui.label("Logarithmic sliders can include infinity and zero.");
          ui.add_space(8.0);
  
          ui.label("If true, the slider will clamp incoming and outgoing values to the given range.");
          ui.label("If false, the slider can shows values outside its range, and you can manually enter values outside the range.");
          ui.add_space(8.0);
  
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

impl Component for SettingWindow {
  fn render(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {} 
}