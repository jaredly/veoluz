fn run(config: shared::Config, outfile: String, count: usize) {
    println!("Calculate");
    let brightness_data = shared::calculate(&config, count);

    println!("Colorize");

    use std::fs::File;
    use std::io::BufWriter;

    let pixels = shared::colorize(&config, &brightness_data);

    if outfile.ends_with(".tiff") {
        let fout = &mut BufWriter::new(File::create(outfile).unwrap());
        image::tiff::TiffEncoder::new(fout)
            .encode(
                &pixels,
                config.rendering.width as u32,
                config.rendering.height as u32,
                image::RGBA(8),
            )
            .unwrap();
    } else {
        // Save the buffer as "image.png"
        image::save_buffer(
            outfile,
            &pixels,
            config.rendering.width as u32,
            config.rendering.height as u32,
            image::RGBA(8),
        )
        .unwrap()
    }
}

pub fn deserialize(encoded: &str) -> Result<shared::Config, serde_json::Error> {
    serde_json::from_str::<shared::Config>(encoded)
        .or_else(|_| serde_json::from_str::<shared::v3::Config>(encoded).map(shared::from_v3))
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

fn load_config(file_name: &str) -> shared::Config {
    let mut file = std::fs::File::open(file_name).expect(&format!("File {} not found", file_name));
    let mut contents = String::new();
    use std::io::prelude::*;
    file.read_to_string(&mut contents).unwrap();
    let config: shared::Config = deserialize(&contents).expect(&format!("Unable to load config file from file {}", file_name));
    config
}

fn resize(config: &mut shared::Config, width: usize, height: usize) {
    let scale = height as f32 / config.rendering.height as f32;
    config.rendering.width = width;
    config.rendering.height = height;
    config.rendering.zoom *= scale;
}

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() -> std::io::Result<()> {

    let matches = App::new("veoluz")
    .version("1.0")
    .author("Jared Forsyth <jared@jaredforsyth.com>")
    .about("See light in a kaleidoscope")
    .arg(Arg::with_name("input_file")
        .help("Indicates the config json file to use")
        .required(true))
    .subcommand(
        SubCommand::with_name("lerp")
            .about("Generate many images to animate between two config files")
            .arg(Arg::with_name("lerp")
                .value_name("second config file")
                .help("A second config file to animate to. When provided, image names will be suffixed, e.g. file_3.png"))
            .arg(Arg::with_name("frames")
                .long("frames")
                .value_name("number of frames")
                .default_value("5")
                .help("The number of images to generate [default: 5]"))
            .arg(Arg::with_name("only")
                .long("only")
                .value_name("frame idx")
                .help("(re)generate a certain index of the animation. Useful for drilling down."))
            .arg(Arg::with_name("genfiles")
                .long("genfiles")
                .help("Write out the generated config files for each frame of the animation.")
            )
    )
    .arg(Arg::with_name("resize")
        .help("Resize the output to the given width,height")
        .long("resize")
        .value_name("width,height"))
    .arg(Arg::with_name("output_file")
        .help("A .png or .tiff file to write to (defaults to the config file + '.png')")
        .short("o")
        .long("output")
        .value_name("some.png"))
    .arg(Arg::with_name("count")
        .short("c")
        .long("count")
        .value_name("number of rays")
        .help("The number of rays to draw")
        .default_value("100000")
        )
    .get_matches()
    ;

    let input_file = matches.value_of("input_file").unwrap().to_owned();
    let mut config = load_config(&input_file);

    let new_size = if let Some(resize) = matches.value_of("resize") {
        let parts: Vec<&str> = resize.split(",").collect();
        if parts.len() != 2 {
            clap::Error::with_description("Argument 'resize' requires a width and height separated by a comma", clap::ErrorKind::InvalidValue).exit();
        }
        let width: usize = parts[0].parse().expect("resize width must be an integer");
        let height: usize = parts[1].parse().expect("resize height must be an integer");
        Some((width, height))
    } else {
        None
    };
    if let Some((width, height)) = new_size {
        resize(&mut config, width, height);
    }
    let outfile = matches.value_of("output_file").unwrap_or(&(input_file + ".png")).to_string();
    let count: usize = matches.value_of("count").unwrap().parse().expect("Count must be an integer");

    if let Some(matches) = matches.subcommand_matches("lerp") {
        let mut config2 = load_config(matches.value_of("lerp").unwrap());
        if let Some((width, height)) = new_size {
            resize(&mut config2, width, height);
        }
        let lerps: usize = matches.value_of("frames").unwrap().parse().unwrap();
        let only: Option<usize> = matches.value_of("only").map(|m|m.parse::<usize>().unwrap());

        use shared::types::LerpEq;
        for i in 0..=lerps {
            if let Some(only) = only {
                if only != i {
                    continue;
                }
            }
            println!("Lerping {}", i);
            let amount = i as f32 / lerps as f32;
            let config = config.lerp(&config2, amount);
            run(config, format!("{}_{}.png", outfile, i), count);
        }
    } else {
        run(config, outfile, count);
    }

    Ok(())
}

// fn main_() -> std::io::Result<()> {
//     // argv
//     let args: Vec<String> = std::env::args().collect();
//     if args[1] == "lerp" {
//         let name: String = args[2].clone();
//         let name2: String = args[3].clone();
//         let outfile: String = args[4].clone();
//         let count: usize = args[5].parse().unwrap();
//         let lerps: usize = args[6].parse().unwrap();
//         let only: Option<usize> = args.get(7).map(|m| m.parse::<usize>().unwrap());

//         let mut config1 = load_config(name);
//         let mut config2 = load_config(name2);
//         resize(&mut config1);
//         resize(&mut config2);

//         use shared::types::LerpEq;
//         for i in 0..=lerps {
//             if let Some(only) = only {
//                 if only != i {
//                     continue;
//                 }
//             }
//             println!("Lerping {}", i);
//             let amount = i as f32 / lerps as f32;
//             let config = config1.lerp(&config2, amount);
//             run(config, format!("{}_{}.tiff", outfile, i), count);
//         }
//     } else if args.len() >= 5 {
//         let name: String = args[1].clone();
//         let outfile: String = args[2].clone();
//         let count: usize = args[3].parse().unwrap();
//         // let scale: u8 = args[4].parse().unwrap();
//         println!("Arg {}", name);

//         let mut config = load_config(name);
//         // config.resize_center(config.rendering.width, config.rendering.width);
//         // config.width *= x;
//         // config.height *= x;
//         // for wall in config.walls.iter_mut() {
//         //     wall.kind.scale(x);
//         // }
//         // for light in config.lights.iter_mut() {
//         //     light.kind.scale(x);
//         // }

//         resize(&mut config);

//         // config.rendering.width *= scale as usize;
//         // config.rendering.height *= scale as usize;
//         // config.rendering.zoom *= scale as f32;

//         run(config, outfile, count);
//     } else {
//         println!("Usage: bin some.json out.png 100000 3")
//     }

//     Ok(())
// }
