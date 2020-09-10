# NOTE: 
This is a temporary fork of https://github.com/DougLau/mvt.  I changed some 
lifetimes and inner workings of the crate.  After spending some more time 
with it, I am looking at sending a PR to the base repository.

# mvt
A library for encoding [mapbox vector tiles](https://github.com/mapbox/vector-tile-spec)
(MVT).  Version 2.1 of the standard is supported.

The API is designed to prevent creating files which are not allowed by the
specification.

## Example (From Doug's Repo). 

```rust
use mvt::{Error, GeomEncoder, GeomType, Tile, Transform};

fn main() -> Result<(), Error> {
    let mut tile = Tile::new(4096);
    let layer = tile.create_layer("First Layer");
    let b = GeomEncoder::new(GeomType::Linestring, Transform::new())
                        .point(0.0, 0.0)
                        .point(1024.0, 0.0)
                        .point(1024.0, 2048.0)
                        .point(2048.0, 2048.0)
                        .point(2048.0, 4096.0)
                        .encode()?;
    let mut feature = layer.into_feature(b);
    feature.set_id(1);
    feature.add_tag_string("key", "value");

    // Changed to address move issue.
    let layer = feature.into_layer();

    tile.add_layer(layer)?;
    let data = tile.to_bytes()?;
    println!("encoded {} bytes: {:?}", data.len(), data);
    Ok(())
}
```

## New Example using lifetimes
```rust
use mvt::{Error, GeomEncoder, GeomType, Tile, Transform};

struct SomePointData {
    name: String,
    x: f64,
    y: f64,
}

impl SomePointData {
    fn new(name: String, x: f64, y: f64) -> Self {
        SomePointData { name, x, y }
    }
}

fn main() -> Result<(), Error> {
    let point_data = vec![
        SomePointData::new("darth".to_string(), -1f64, -1f64),
        SomePointData::new("kylo".to_string(), -1f64, -0f64),
        SomePointData::new("luke".to_string(), 1f64, 1f64),
    ];

    let mut tile = Tile::new(4096);
    let mut layer = tile.create_layer("First Layer");

    let mut fcount = 0;
    for pd in point_data {
        let b = GeomEncoder::new(GeomType::Point, Transform::new())
            .point(pd.x, pd.y)
            .encode()?;
        let mut feature = layer.into_feature(b);
        fcount += 1;
        feature.set_id(fcount);
        feature.add_tag_string("name", &pd.name);

        // Closes/Commits the feature to the layer.
        feature.into_layer();
    }
    tile.add_layer(layer)?;

    let data = tile.to_bytes()?;
    println!("encoded {} bytes: {:?}", data.len(), data);
    Ok(())
}
```

## Alternatives

These are other rust projects with MVT support:
* [hecate](https://crates.io/crates/hecate)
* [t-rex](https://t-rex.tileserver.ch/)
* [vectortile](https://crates.io/crates/vectortile)
