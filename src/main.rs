use std::fs::File;
use std::io::BufReader;
use serde_json::{Value};
use crate::scene::Scene;

mod scene;
mod util;
mod surfaces;
mod material;

fn main() {
    let scene_file = File::open("./scenes/plane.json").expect("Error opening scene file");
    let reader = BufReader::new(scene_file);
    let scene_json: Value = serde_json::from_reader(reader).expect("Error reading scene file");

    let scene_m = Scene::parse_from_json(&scene_json);

    let image = scene_m.ray_trace_image();
    image.save("output_image.png").unwrap();
}
