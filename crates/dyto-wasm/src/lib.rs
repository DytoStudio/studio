use dyto_scene;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct DytoScene(dyto_scene::DytoScene);

#[wasm_bindgen]
impl DytoScene {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_selector: &str) -> DytoScene {
        DytoScene(dyto_scene::DytoScene::new(canvas_selector))
    }

    pub fn run(self) {
        self.0.run();
    }
}
