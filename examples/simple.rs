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
        feature.into_layer();
    }
    tile.add_layer(layer)?;

    let data = tile.to_bytes()?;
    println!("encoded {} bytes: {:?}", data.len(), data);
    Ok(())
}
