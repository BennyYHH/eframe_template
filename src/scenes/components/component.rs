pub trait Component {
  fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
}