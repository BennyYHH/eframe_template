use crate::scenes::components::Component;

pub trait WindowComponent: Component {
    fn render(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame, open: &mut bool);

    fn get_id(&self) -> &str;

    fn get_button_name(&self) -> &str;

    fn get_window_name(&self) -> &str;
}