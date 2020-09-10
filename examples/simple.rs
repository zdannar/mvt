use mvt::{GeomEncoder, GeomType, Tile, Transform};
//extern crate slippy_map_tilenames as smt;

struct SomePointData {
    name: String,
    x: f64,
    y: f64,
}

impl SomePointData {
    fn new(name: String, x: f64, y: f64) -> Self {
        SomePointData { name, x, y }
    }

    /*
    fn from_latlon(&self, zoom: u8) -> Self {
        // If you add crate slippy_map_tilenames, you can use lat/lon EPSG 4326
        // Points, you can get the appropriate tile values.
        let (x, y) = smt::lonlat2tile(self.x, self.y, zoom);
        SomePointData {
            name: self.name.clone(),
            x: x.into(),
            y: y.into(),
        }
    }
    */
}

fn make_tile(point_data: Vec<SomePointData>) -> Result<Vec<u8>, mvt::Error> {
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
    Ok(data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let point_data = vec![
        SomePointData::new("darth".to_string(), -1f64, -1f64),
        SomePointData::new("kylo".to_string(), -1f64, -0f64),
        SomePointData::new("luke".to_string(), 1f64, 1f64),
        SomePointData::new("yoda".to_string(), 4f64, 1f64),
    ];
    let bdata = make_tile(point_data)?;
    println!("Bytes: {:?}", bdata);
    Ok(())
}
