// lib.rs      mvt crate.
//
// Copyright (c) 2019 Minnesota Department of Transportation
//
//! A library for encoding [mapbox vector tiles](https://github.com/mapbox/vector-tile-spec)
//! (MVT).
//!
//! A [tile](struct.Tile.html) is composed of one or more
//! [layer](struct.Layer.html)s.  Each layer can have any number of
//! [feature](struct.Feature.html)s, which contain the geometry to be rendered.
//! They can also have metadata tags, which are key/value pairs.
//!
//! ## Example
//!
//! ```rust
//! use mvt::{Error, GeomEncoder, GeomType, Tile, Transform};
//!
//! fn main() -> Result<(), Error> {
//!     let mut tile = Tile::new(4096);
//!     let mut layer = tile.create_layer("First Layer");
//!     let b = GeomEncoder::new(GeomType::Linestring, Transform::new())
//!                         .point(0.0, 0.0)
//!                         .point(1024.0, 0.0)
//!                         .point(1024.0, 2048.0)
//!                         .point(2048.0, 2048.0)
//!                         .point(2048.0, 4096.0)
//!                         .encode()?;
//!     let mut feature = layer.into_feature(b);
//!     feature.set_id(1);
//!     feature.add_tag_string("key", "value");
//!     feature.into_layer();
//!     tile.add_layer(layer)?;
//!     let data = tile.to_bytes()?;
//!     println!("encoded {} bytes: {:?}", data.len(), data);
//!     Ok(())
//! }
//! ```
#[macro_use]
extern crate log;

mod encoder;
mod error;
mod geom;
mod mapgrid;
mod tile;
mod vector_tile;

pub use crate::encoder::{GeomData, GeomEncoder, GeomType};
pub use crate::error::Error;
pub use crate::geom::{Transform, Vec2};
pub use crate::mapgrid::{BBox, MapGrid, TileId};
pub use crate::tile::{Feature, Layer, Tile};
