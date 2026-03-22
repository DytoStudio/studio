use dyto_scene::scene::Scene;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DytoScene(Scene);

#[wasm_bindgen]
impl DytoScene {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_selector: &str) -> DytoScene {
        DytoScene(Scene::new(canvas_selector))
    }

    pub fn run(&mut self) {
        self.0.run();
    }
}
