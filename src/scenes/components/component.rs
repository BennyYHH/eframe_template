pub trait Component {
  fn render(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame);
}