//! The module for handling messages between the scene and the outside world.
use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        message::Messages,
        resource::Resource,
        system::{Commands, Res},
    },
};
use crossbeam_channel::Receiver;

use crate::image_tile::mercator;

pub mod messages;

pub use messages::EmbedderToSceneMessages;

/// A message that is sent from the scene to the embedder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SceneToEmbedderMessages {
    /// A message to request an image tile.
    RequestImageTile(Box<[mercator::TileCoordinate]>),
}

/// Resource for the message bridge.
#[derive(Resource)]
pub struct MessageBridge {
    /// The receiver for messages from the embedder to the scene.
    pub receiver: Receiver<EmbedderToSceneMessages>,
    /// The function to send messages from the scene to the embedder.
    pub send_fn: Box<dyn Fn(SceneToEmbedderMessages) + Send + Sync>,
}

/// The plugin for the message bridge.
pub struct MessageBridgePlugin;

/// A function to dispatch messages from the embedder to the scene.
fn dispatch_messages(mut commands: Commands, bridge: Res<MessageBridge>) {
    for message in bridge.receiver.try_iter() {
        bevy::log::debug!("Received message from embedder: {:?}", message);
        match message {
            EmbedderToSceneMessages::ImageTileLoaded(image_tile_loaded) => {
                commands.write_message(image_tile_loaded)
            }
        };
    }
}

impl Plugin for MessageBridgePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Messages<messages::ImageTileLoaded>>();
        app.add_systems(Update, dispatch_messages);

        let (_sender, receiver) = crossbeam_channel::unbounded();

        app.insert_resource(MessageBridge {
            receiver,
            send_fn: Box::new(move |_message| ()),
        });
    }
}
