#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};

use crossbeam_channel::unbounded;
use dyto_scene::{
    image_tile::mercator,
    message_bridge::{
        EmbedderToSceneMessages, SceneToEmbedderMessages, messages,
    },
    scene::Scene,
};
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
struct JSFns {
    on_image_tile_requested: Option<js_sys::Function>,
}

#[wasm_bindgen]
pub struct DytoScene {
    scene: Scene,
    running: bool,
    sender: crossbeam_channel::Sender<EmbedderToSceneMessages>,
    fns: Arc<Mutex<JSFns>>,
}

#[wasm_bindgen]
impl DytoScene {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_selector: &str) -> DytoScene {
        let (sender, receiver) = unbounded::<EmbedderToSceneMessages>();
        let mut scene = Scene::new(canvas_selector);

        let fns = Arc::new(Mutex::new(JSFns {
            on_image_tile_requested: None,
        }));
        let fns_clone = fns.clone();

        scene.setup_message_bridge(
            receiver,
            Box::new(move |message| {
                let fns = fns_clone.clone();
                Self::handle_message(fns, message);
            }),
        );

        Self {
            scene,
            running: false,
            sender,
            fns,
        }
    }

    fn handle_message(
        fns: Arc<Mutex<JSFns>>,
        message: SceneToEmbedderMessages,
    ) {
        let Ok(fns) = fns.lock() else {
            return;
        };
        match message {
            SceneToEmbedderMessages::RequestImageTile(tile_coordinates) => {
                if let Some(on_image_tile_requested) =
                    &fns.on_image_tile_requested
                {
                    let this = JsValue::NULL;
                    let mut data: Vec<u32> =
                        Vec::with_capacity(tile_coordinates.len() * 3);
                    for tile_coordinate in tile_coordinates.iter() {
                        data.push(tile_coordinate.x());
                        data.push(tile_coordinate.y());
                        data.push(
                            tile_coordinate.level_of_detail().value() as u32
                        );
                    }
                    let js_array = js_sys::Uint32Array::from(&data[..]);
                    let _ = on_image_tile_requested.call1(&this, &js_array);
                }
            }
        }
    }

    pub fn run(&mut self) {
        self.scene.run();
        self.running = true;
    }

    #[wasm_bindgen(setter, js_name = "onImageTileRequested")]
    pub fn set_on_image_tile_requested(&mut self, callback: js_sys::Function) {
        let Ok(mut fns) = self.fns.lock() else {
            return;
        };
        fns.on_image_tile_requested = Some(callback);
    }

    #[wasm_bindgen(js_name = "sendImageTileLoaded")]
    pub fn send_image_tile_loaded(
        &mut self,
        tile_coordinate: js_sys::Uint32Array,
        image_data: js_sys::Uint8Array,
    ) {
        let tile_coordinate_array = tile_coordinate.to_vec();
        if tile_coordinate_array.len() != 3 {
            error!(
                "Invalid tile coordinate array length: expected 3, got {}",
                tile_coordinate_array.len()
            );
            return;
        }
        let tile_coordinate = mercator::TileCoordinate::new(
            tile_coordinate_array[0],
            tile_coordinate_array[1],
            mercator::LevelOfDetail::new(tile_coordinate_array[2] as u8),
        );
        let image_data_vec = image_data.to_vec();
        let _ = self.sender.send(EmbedderToSceneMessages::ImageTileLoaded(
            messages::ImageTileLoaded(
                tile_coordinate,
                Arc::from(image_data_vec),
            ),
        ));
    }
}
