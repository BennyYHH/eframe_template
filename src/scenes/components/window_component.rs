use super::component::Component;

pub trait WindowComponent: Component {
    fn render(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);

    fn get_id(&self) -> String;

    fn get_button_name(&self) -> String;

    fn get_window_name(&self) -> String;
}