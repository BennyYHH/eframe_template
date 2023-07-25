use super::components::LeftBarMenu;
use super::components::Component;
use super::scene::Scene;

pub struct MainScene {
  name: String,
  id: u32,
  components: Vec<Box<dyn Component>>,
}

impl Default for MainScene {
  fn default() -> Self {
    Self {
      name: "Hello World!".to_owned(),
      id: 1,
      components: vec![
        Box::new(LeftBarMenu::default()),
      ],
    }
  }
}

impl Scene for MainScene {
  fn get_name(&self) -> String {
    self.name.clone()
  }
  fn get_id(&self) -> u32 {
    self.id
  }
  fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    for component in &mut self.components {
      component.render(ctx, _frame);
    }
  }
}