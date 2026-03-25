//! A module for receiving messages from the embedder.
use std::sync::Arc;

use bevy::ecs::message::Message;

use crate::image_tile::mercator;

/// A message to indicate that an image tile has been loaded.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct ImageTileLoaded(pub mercator::TileCoordinate, pub Arc<[u8]>);

/// A message that is sent from the embedder to the scene.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EmbedderToSceneMessages {
    /// A message to indicate that an image tile has been loaded.
    ImageTileLoaded(ImageTileLoaded),
}
