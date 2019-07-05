
fn run(config: shared::Config, outfile: String, count: usize) {
    println!("Calculate");
    let brightness_data = shared::calculate(&config, count);

    println!("Colorize");

    use std::fs::File;
    use std::io::BufWriter;

    let pixels = shared::colorize(&config, &brightness_data);

    if outfile.ends_with(".tiff") {
        let fout = &mut BufWriter::new(File::create(outfile).unwrap());
        image::tiff::TiffEncoder::new(fout).encode(
            &pixels,
        config.rendering.width as u32, config.rendering.height as u32, image::RGBA(8)
        ).unwrap();
    } else {
        // Save the buffer as "image.png"
        image::save_buffer(outfile, &pixels, config.rendering.width as u32, config.rendering.height as u32, image::RGBA(8)).unwrap()
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

fn load_config(file_name: String) -> shared::Config {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    use std::io::prelude::*;
    file.read_to_string(&mut contents).unwrap();
    let config: shared::Config = deserialize(&contents).unwrap();
    config
}

fn resize(config: &mut shared::Config) {
    let edge = 8 * 200;
    let scale = edge as f32 / config.rendering.height as f32;

    // Square it off y'all
    config.rendering.width = edge;
    config.rendering.height = edge;
    config.rendering.zoom *= scale;
}

fn main() -> std::io::Result<()> {
    // argv
    let args: Vec<String> = std::env::args().collect();
    if args[1] == "lerp" {
        let name: String = args[2].clone();
        let name2: String = args[3].clone();
        let outfile: String = args[4].clone();
        let count: usize = args[5].parse().unwrap();
        let lerps: usize = args[6].parse().unwrap();

        let mut config1 = load_config(name);
        let mut config2 = load_config(name2);
        resize(&mut config1);
        resize(&mut config2);

        use shared::types::LerpEq;

        for i in 0..=lerps {
            println!("Lerping {}", i);
            let amount = i as f32 / lerps as f32;
            let config = config1.lerp(&config2, amount);
            run(config, format!("{}_{}.png", outfile, i), count);
        }
    } else if args.len() >= 5 {
        let name: String = args[1].clone();
        let outfile: String = args[2].clone();
        let count: usize = args[3].parse().unwrap();
        // let scale: u8 = args[4].parse().unwrap();
        println!("Arg {}", name);

        let mut config = load_config(name);
        // config.resize_center(config.rendering.width, config.rendering.width);
        // config.width *= x;
        // config.height *= x;
        // for wall in config.walls.iter_mut() {
        //     wall.kind.scale(x);
        // }
        // for light in config.lights.iter_mut() {
        //     light.kind.scale(x);
        // }

        resize(&mut config);

        // config.rendering.width *= scale as usize;
        // config.rendering.height *= scale as usize;
        // config.rendering.zoom *= scale as f32;

        run(config, outfile, count);

    } else {
        println!("Usage: bin some.json out.png 100000 3")
    }

    Ok(())
}
