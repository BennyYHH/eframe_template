use crate::scenes::{landing_scene::LandingScene, scene::Scene, main_scene::MainScene};
use std::collections::HashMap;


pub struct SceneManager {
    pub scenes: HashMap<u32, Box<dyn Scene>>,
    pub current_scene_id: u32,
}

impl Default for SceneManager {
    fn default() -> Self {
        let mut scenes: HashMap<u32, Box<dyn Scene>> = HashMap::new();
        let landing_scene = Box::new(LandingScene::default());
        scenes.insert(landing_scene.get_id(), landing_scene);
        let main_scene = Box::new(MainScene::default());
        scenes.insert(main_scene.get_id(), main_scene);
        Self {
            scenes: scenes,
            current_scene_id: 0,
        }
    }
}

impl SceneManager {
    #[allow(dead_code)]
    pub fn get_current_scene(&self) -> &Box<dyn Scene> {
        self.scenes.get(&self.current_scene_id).unwrap()
    }

    #[allow(dead_code)]
    pub fn render(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.scenes.get_mut(&self.current_scene_id).unwrap().render(ctx, _frame);
    }
}
