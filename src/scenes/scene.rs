use super::components::Component;
use super::components::LeftBarMenu;


pub struct Scene {
  name: String,
  id: u32,

  components: Vec<Box<dyn Component>>,
}

impl Default for Scene {
  fn default() -> Self {
    Self {
      name: "Hello World!".to_owned(),
      id: 2,
      components: vec![
        Box::new(LeftBarMenu::default()),
      ],
    }
  }
}

impl Scene {
  pub fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    for component in &mut self.components {
      component.render(ctx, _frame);
    }
  }
}