use crate::line;
use crate::types::*;
use crate::Timer;

pub fn grayscale(config: &Config, brightness_data: &[line::uint], scale: u8) -> Vec<u8> {
    let _timer = Timer::new("Grayscale");
    let width = config.rendering.width * scale as usize;
    let height = config.rendering.height * scale as usize;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    let expose = exposer(&config.rendering.exposure);

    let mut data = vec![0; width * height];
    let top = top as line::float;
    for x in 0..width {
        for y in 0..height {
            let index = x + y * width;
            let brightness = brightness_data[x + y * width];
            data[index] = expose(top, brightness) as u8;
        }
    }

    data
}

fn exposer<'a>(exposure: &Exposure) -> Box<Fn(line::float, line::uint) -> f32> {
    let min: line::float = exposure.min;
    let max: line::float = exposure.max;
    // if it's 0 - 0.75
    // we want (x * (255.0 / .75)).max(255.0)
    // if it's 0.25 - 0.75
    // we want ((x - .25).min(0.0) * 255.0 / .5).max(255.0)
    let scale: line::float = 255.0 / (max - min);
    let scaler = move |amt: line::float| ((amt - min).max(0.0) * scale).min(255.0);
    match exposure.curve {
        Curve::FourthRoot => {
            Box::new(move |top: line::float, brightness: line::uint| {
                scaler((brightness as line::float / top).sqrt().sqrt())
            })
            // (((brightness as line::float / top).sqrt().sqrt() - min).min(0.0) * scale).max(255.0) as u8
        }
        // FourthRoot => move |top, brightness|
        //     scaler((brightness as line::float / top).sqrt().sqrt()),
        Curve::SquareRoot => {
            Box::new(move |top: line::float, brightness: line::uint| {
                scaler((brightness as line::float / top).sqrt())
            })
            // (((brightness as line::float / top).sqrt().sqrt() - min).min(0.0) * scale).max(255.0) as u8;
            // move |top, brightness: u8|
            // scaler((brightness as line::float / top).sqrt())
        }
        Curve::Linear => Box::new(move |top, brightness| scaler(brightness as line::float / top)),
    }
}

// #[inline]
// fn expose(top: line::float, brightness: line::uint) -> u8 {
//     // ((brightness as line::float / top).sqrt().sqrt() * 255.0) as u8
//     // ((brightness as line::float / top).sqrt() * 255.0) as u8
//     // ((brightness as line::float / top).sqrt() * 500.0).min(255.0) as u8
//     // (brightness as line::float / top * 1000.0).min(255.0) as u8
//     (brightness as line::float / top * 4000.0).min(255.0) as u8
// }

pub fn histogram(config: &Config, brightness_data: &[line::uint], bin_count: usize) -> Vec<u32> {
    let width = config.rendering.width as usize;
    let height = config.rendering.height as usize;

    let mut bins = vec![0; bin_count];

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    
    let expose = exposer(&config.rendering.exposure);

    let top = top as line::float;
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            let exposed = expose(top, brightness) / 255.0 * (bin_count - 1) as f32;
            bins[exposed as usize] += 1;
        }
    }

    bins
}

pub fn colorize(config: &Config, brightness_data: &[line::uint], scale: u8) -> Vec<u8> {
    // let _timer = Timer::new("Colorize");
    let width = config.rendering.width * scale as usize;
    let height = config.rendering.height * scale as usize;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }

    // let front = (255.0, 255.0, 255.0);
    // let back = (0.0, 50.0, 0.0);
    let color: Box<Fn(f32, &mut [u8], usize)> = match config.rendering.coloration {
        Coloration::Rgb {
            highlight,
            background,
        } => {
            // (
            //     (highlight.0 as f32, highlight.1 as f32, highlight.2 as f32),
            //     (
            //         background.0 as f32,
            //         background.1 as f32,
            //         background.2 as f32,
            //     ),
            // );
            let back = background;
            let front = highlight;
            Box::new(move |exposed, data: &mut [u8], index| {
                data[index] = (front.0 as f32 * exposed + back.0 as f32 * (1.0 - exposed)) as u8;
                data[index + 1] = (front.1 as f32 * exposed + back.1 as f32 * (1.0 - exposed)) as u8;
                data[index + 2] = (front.2 as f32 * exposed + back.2 as f32 * (1.0 - exposed)) as u8;
            })
        },
        Coloration::HueRange { 
            start,
            end,
            saturation,
            lightness
         } => {
             let range = (end - start) as f64;
             let start = start as f64;
             Box::new(move |exposed, data: &mut [u8], index| {
                 let hue = start + range * exposed as f64;
                 let hsl = colorsys::Hsl::new(hue, saturation as f64, lightness as f64, None);
                 let rgb: colorsys::Rgb = hsl.into();
                data[index] = rgb.get_red() as u8;
                data[index + 1] = rgb.get_green() as u8;
                data[index + 2] = rgb.get_blue() as u8;
             })
         }
    };
    // let front = (131.0, 43.0, 93.0);
    // let front = (255.0, 200.0, 230.0);

    let expose = exposer(&config.rendering.exposure);

    let mut data = vec![0; width * height * 4];
    let top = top as line::float;
    // let scale =
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            let exposed = expose(top, brightness) / 255.0;
            color(exposed, &mut data, index);
            // data[index] = (front.0 * exposed + back.0 * (1.0 - exposed)) as u8;
            // data[index + 1] = (front.1 * exposed + back.1 * (1.0 - exposed)) as u8;
            // data[index + 2] = (front.2 * exposed + back.2 * (1.0 - exposed)) as u8;
            data[index + 3] = 255;
        }
    }

    data
}

pub fn black_colorize(config: &Config, brightness_data: &[line::uint], scale: u8) -> Vec<u8> {
    // something like 5% of the time is here
    // let _timer = Timer::new("Colorize");
    let width = config.rendering.width * scale as usize;
    let height = config.rendering.height * scale as usize;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    let expose = exposer(&config.rendering.exposure);

    let mut data = vec![0; width * height * 4];
    let top = top as line::float;
    // let scale =
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            let bright = expose(top, brightness) as u8;
            data[index] = bright;
            data[index + 1] = bright;
            data[index + 2] = bright;
            data[index + 3] = 255;
        }
    }

    data
}

pub fn zen_photon(config: &Config) -> Vec<u8> {
    let brightness_data = crate::calculate::calculate(&config, 100_000, 1);

    colorize(&config, &brightness_data, 1)
}
