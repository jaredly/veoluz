
fn run(config: shared::Config, outfile: String, count: usize) {
    println!("Calculate");
    let brightness_data = shared::calculate(&config, count);
    println!("Colorize");
    let pixels = shared::black_colorize(&config, &brightness_data);

    // For reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;
    // To use encoder.set()
    use png::HasParameters;

    println!("Write out");
    let path = Path::new(&outfile);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, config.width as u32, config.height as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap(); // Save
}

fn main() -> std::io::Result<()> {
    // argv
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 4 {
        let name: String = args[1].clone();
        let outfile: String = args[2].clone();
        let count: usize = args[3].parse().unwrap();
        println!("Arg {}", name);

        let mut file = std::fs::File::open(name)?;
        let mut contents = String::new();

        use std::io::prelude::*;
        file.read_to_string(&mut contents)?;
        let config: shared::Config = serde_json::from_str(&contents).unwrap();

        run(config, outfile, count);

    } else {
        println!("Usage: bin some.json out.png 100000")
    }

    Ok(())
}
