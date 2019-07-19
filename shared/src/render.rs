use crate::line;
use crate::types::*;
use crate::Timer;

pub fn grayscale(config: &Config, brightness_data: &[line::uint]) -> Vec<u8> {
    let _timer = Timer::new("Grayscale");
    let width = config.rendering.width;
    let height = config.rendering.height;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    let expose = exposer(&config.rendering.exposure, config, brightness_data);

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

// #[inline]
// fn expose(exposure: &Exposure, top: line::float, brightness: line::uint) -> f32 {
//     let min: line::float = exposure.min;
//     let max: line::float = exposure.max.max(min + 0.01);
//     let scale: line::float = 255.0 / (max - min);
//     let amt = match exposure.curve {
//         Curve::FourthRoot => ((brightness as line::float / top).sqrt().sqrt()),
//         Curve::SquareRoot => ((brightness as line::float / top).sqrt()),
//         Curve::Linear => (brightness as line::float / top),
//     };
//     ((amt - min).max(0.0) * scale).min(255.0)
// }

fn exposer2(
    exposure: &Exposure,
    min: line::float,
    max: line::float,
) -> Box<Fn(line::float, line::uint) -> f32> {
    let max: line::float = max.max(min + 0.01);
    let scale: line::float = 255.0 / (max - min);
    let scaler = move |amt: line::float| ((amt - min).max(0.0) * scale).min(255.0);
    match exposure.curve {
        Curve::FourthRoot => Box::new(move |top: line::float, brightness: line::uint| {
            scaler((brightness as line::float / top).sqrt().sqrt())
        }),
        Curve::SquareRoot => Box::new(move |top: line::float, brightness: line::uint| {
            scaler((brightness as line::float / top).sqrt())
        }),
        Curve::Linear => Box::new(move |top, brightness| scaler(brightness as line::float / top)),
    }
}

fn exposer(
    exposure: &Exposure,
    config: &Config,
    brightness_data: &[line::uint],
) -> Box<Fn(line::float, line::uint) -> f32> {
    match exposure.limits {
        Some((min, max)) => exposer2(exposure, min, max),
        None => {
            let (min, max) = hist_max(config, brightness_data);
            exposer2(exposure, min, max)
        }
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

    let expose = exposer(&config.rendering.exposure, config, brightness_data);

    let top = top as line::float;
    for x in 0..width {
        for y in 0..height {
            let brightness = brightness_data[x + y * width];
            let exposed = expose(top, brightness) / 255.0 * (bin_count - 1) as f32;
            bins[exposed as usize] += 1;
        }
    }

    bins
}

#[inline]
fn blend(front: u8, back: u8, front_alpha: f32) -> f32 {
    // let gamma = 2.2;
    let front = front as f32 / 255.0;
    let back = back as f32 / 255.0;
    // let front = front.powf(gamma);
    // let back = back.powf(gamma);

    let res = front * front_alpha + back * (1.0 - front_alpha);
    // let res = res.powf(1.0 / gamma);

    res * 255.0
}

pub fn hist_max(config: &Config, brightness_data: &[line::uint]) -> (line::float, line::float) {
    let width = config.rendering.width as usize;
    let height = config.rendering.height as usize;

    let mut top = 0.0 as f32;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width] as f32);
        }
    }
    let mut bins = vec![0; 100];

    for x in 0..width {
        for y in 0..height {
            let brightness = brightness_data[x + y * width];

            let amt = match config.rendering.exposure.curve {
                Curve::FourthRoot => ((brightness as line::float / top).sqrt().sqrt()),
                Curve::SquareRoot => ((brightness as line::float / top).sqrt()),
                Curve::Linear => (brightness as line::float / top),
            };

            let exposed = amt * (100 - 1) as f32;
            bins[exposed as usize] += 1;
        }
    }

    let low_end = ((width * height) as f32 * 0.00001) as usize;
    let low_end = ((width * height) as f32 * 0.50) as usize;

    let mut low = 0.0;

    let ninetyfive = ((width * height) as f32 * 0.9999) as usize;
    let mut covered = 0;
    for i in 0..100 {
        covered += bins[i];
        // Ummm not sure how to do the auto low-end -- this didn't work super well
        // if covered < low_end {
        //     low = i as f32 / 100.0;
        // }
        if covered > ninetyfive {
            return (low, i as f32 / 100.0);
        }
    }

    (low, 1.0)
}

pub fn colorize(config: &Config, brightness_data: &[line::uint]) -> Vec<u8> {
    // let _timer = Timer::new("Colorize");
    let width = config.rendering.width;
    let height = config.rendering.height;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }

    let color: Box<Fn(f32, &mut [u8], usize)> = match config.rendering.coloration {
        Coloration::Rgb {
            highlight,
            background,
        } => {
            let back = background;
            let front = highlight;
            Box::new(move |exposed, data: &mut [u8], index| {
                data[index] = blend(front.0, back.0, exposed) as u8;
                data[index + 1] = blend(front.1, back.1, exposed) as u8;
                data[index + 2] = blend(front.2, back.2, exposed) as u8;
            })
        }
        Coloration::HueRange {
            start,
            end,
            saturation,
            lightness,
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

    let expose = exposer(&config.rendering.exposure, config, brightness_data);

    let mut data = vec![0; width * height * 4];
    let top = top as line::float;
    // let scale =
    for x in 0..width {
        for y in 0..height {
            let index = (x + y * width) * 4;
            let brightness = brightness_data[x + y * width];
            let exposed = expose(top, brightness) / 255.0;
            color(exposed, &mut data, index);
            data[index + 3] = 255;
        }
    }

    data
}

pub fn black_colorize(config: &Config, brightness_data: &[line::uint]) -> Vec<u8> {
    // something like 5% of the time is here
    // let _timer = Timer::new("Colorize");
    let width = config.rendering.width;
    let height = config.rendering.height;

    let mut top = 0;
    for x in 0..width {
        for y in 0..height {
            top = top.max(brightness_data[x + y * width]);
        }
    }
    let expose = exposer(&config.rendering.exposure, config, brightness_data);

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
    let brightness_data = crate::calculate::calculate(&config, 100_000);

    colorize(&config, &brightness_data)
}
