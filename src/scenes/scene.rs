pub trait Scene {
  fn get_name(&self) -> String;

  fn get_id(&self) -> u32;

  fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame);
}
