
fn run(config: shared::Config, outfile: String, count: usize) {
    println!("Calculate");
    let brightness_data = shared::calculate(&config, count);
    println!("Colorize");
    // let pixels = shared::black_colorize(&config, &brightness_data, scale);

    // // For reading and opening files
    // use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    // // To use encoder.set()
    // use png::HasParameters;

    // println!("Write out");
    // let path = Path::new(&outfile);
    // let file = File::create(path).unwrap();
    // let ref mut w = BufWriter::new(file);

    // let mut encoder = png::Encoder::new(w, config.width as u32 * scale as u32, config.height as u32 * scale as u32); // Width is 2 pixels and height is 1.
    // encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    // let mut writer = encoder.write_header().unwrap();

    // writer.write_image_data(&pixels).unwrap(); // Save

    let pixels = shared::grayscale(&config, &brightness_data);

    if outfile.ends_with(".tiff") {
        let fout = &mut BufWriter::new(File::create(outfile).unwrap());
        image::tiff::TiffEncoder::new(fout).encode(
            &pixels,
        config.rendering.width as u32, config.rendering.height as u32, image::Gray(8)
        ).unwrap();
    } else {
        // Save the buffer as "image.png"
        image::save_buffer(outfile, &pixels, config.rendering.width as u32, config.rendering.height as u32, image::Gray(8)).unwrap()
    }
}

pub fn deserialize(encoded: &str) -> Result<shared::Config, serde_json::Error> {
    serde_json::from_str::<shared::Config>(encoded)
        .or_else(|_| {
            serde_json::from_str::<shared::v3::Config>(encoded)
                .map(shared::from_v3)
        })
        .or_else(|_| {
            serde_json::from_str::<shared::v2::Config>(encoded)
                .map(shared::v3::from_v2)
                .map(shared::from_v3)
        })
        .or_else(|_| {
            serde_json::from_str::<shared::v1::Config>(encoded)
                .map(shared::v2::from_v1)
                .map(shared::v3::from_v2)
                .map(shared::from_v3)
        })
}

fn main() -> std::io::Result<()> {
    // argv
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 5 {
        let name: String = args[1].clone();
        let outfile: String = args[2].clone();
        let count: usize = args[3].parse().unwrap();
        let scale: u8 = args[4].parse().unwrap();
        println!("Arg {}", name);

        let mut file = std::fs::File::open(name)?;
        let mut contents = String::new();

        use std::io::prelude::*;
        file.read_to_string(&mut contents)?;
        let mut config: shared::Config = deserialize(&contents).unwrap();
        // config.resize_center(config.rendering.width, config.rendering.width);
        // config.width *= x;
        // config.height *= x;
        // for wall in config.walls.iter_mut() {
        //     wall.kind.scale(x);
        // }
        // for light in config.lights.iter_mut() {
        //     light.kind.scale(x);
        // }
        config.rendering.width *= scale as usize;
        config.rendering.height *= scale as usize;
        config.rendering.zoom *= scale as f32;

        run(config, outfile, count);

    } else {
        println!("Usage: bin some.json out.png 100000 3")
    }

    Ok(())
}
