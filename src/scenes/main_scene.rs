use super::components::LeftBarMenu;
use super::components::Component;
use super::scene::Scene;

pub struct MainScene {
  id: u32,
  components: Vec<Box<dyn Component>>,
}

impl Default for MainScene {
  fn default() -> Self {
    Self {
      id: 1,
      components: vec![
        Box::new(LeftBarMenu::default()),
      ],
    }
  }
}

impl Scene for MainScene {
  fn get_id(&self) -> u32 {
    self.id
  }
  fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    for component in &mut self.components {
      component.render(ctx, _frame);
    }
  }
}