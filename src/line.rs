extern crate num_traits;
use num_traits::{Float, NumAssignOps, NumCast, Signed};
use std::mem::swap;

pub type float = f32;
pub type int = i16;
pub type uint = u16;
pub const PI: float = std::f32::consts::PI;

#[inline]
fn draw(data: &mut [uint], width: usize, full: float, x: int, y: int, amount: float) {
    let index = (y as usize) * width + x as usize;
    let brightness = (amount * full) as uint;
    data[index] += brightness;
}

pub fn draw_line(
    mut start: Point<float>,
    mut end: Point<float>,
    data: &mut [uint],
    width: usize,
    height: usize,
    full: float,
) {
    // -- The Setup part

    let steep = (end.1 - start.1).abs() > (end.0 - start.0).abs();

    if steep {
        start = (start.1, start.0);
        end = (end.1, end.0);
    }

    if start.0 > end.0 {
        swap(&mut start, &mut end);
    }
    // log!("Line xs = {} - {}", start.0, end.0);

    let mut gradient = (end.1 - start.1) / (end.0 - start.0);

    if gradient == 0.0 {
        gradient = 1.0;
    }

    // -- the algorithm

    let mut y = start.1;

    /*
    for x in start.0.round() as int..end.0.round() as int {
        let fpart = y.fract();
        let yy = y as int;

        // Get the point
        let point = if steep { (yy, x) } else { (x, yy) };

        // TODO get rid of these if blocks by ensuring we're in scope to start with
        if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int {
            draw(data, width, full, point.0, point.1, 1.0 - fpart)
        }

        if fpart > 0.0 {
            let point = if steep { (yy + 1, x) } else { (x, yy + 1) };
            if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int {
                draw(data, width, full, point.0, point.1, fpart)
            }
        }
        y += gradient;
    }
    */

    if steep {
        for x in start.0.round() as int..end.0.round() as int {
            let fpart = y.fract();
            let yy = y as int;

            // Get the point
            let point = (yy, x);

            // TODO get rid of these if blocks by ensuring we're in scope to start with
            if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int {
                draw(data, width, full, point.0, point.1, 1.0 - fpart)
            }

            if fpart > 0.0 {
                let point = (yy + 1, x);
                if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int
                {
                    draw(data, width, full, point.0, point.1, fpart)
                }
            }
            y += gradient;
        }
    } else {
        for x in start.0.round() as int..end.0.round() as int {
            let fpart = y.fract();
            let yy = y as int;

            // Get the point
            let point = (x, yy);

            // TODO get rid of these if blocks by ensuring we're in scope to start with
            if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int {
                draw(data, width, full, point.0, point.1, 1.0 - fpart)
            }

            if fpart > 0.0 {
                let point = (x, yy + 1);
                if point.0 >= 0 && point.1 >= 0 && point.0 < width as int && point.1 < height as int
                {
                    draw(data, width, full, point.0, point.1, fpart)
                }
            }
            y += gradient;
        }
    }
}

/// An implementation of [Xiaolin Wu's line algorithm].
///
/// This algorithm works based on floating-points and returns an extra variable for how much a
/// a point is covered, which is useful for anti-aliasing.
///
/// Note that due to the implementation, the returned line will always go from left to right.
///
/// Example:
///
/// ```
/// extern crate line_drawing;
/// use line_drawing::XiaolinWu;
///
/// fn main() {
///     for ((x, y), value) in XiaolinWu::<float, int>::new((0.0, 0.0), (3.0, 6.0)) {
///         print!("(({}, {}), {}), ", x, y, value);
///     }
/// }
/// ```
///
/// ```text
/// ((0, 0), 0.5), ((0, 1), 0.5), ((1, 1), 0.5), ((1, 2), 1), ((1, 3), 0.5), ((2, 3), 0.5), ((2, 4), 1), ((2, 5), 0.5), ((3, 5), 0.5), ((3, 6), 0.5),
/// ```
///
/// [Xiaolin Wu's line algorithm]: https://en.wikipedia.org/wiki/Xiaolin_Wu%27s_line_algorithm
pub struct XiaolinWu<I, O> {
    steep: bool,
    gradient: I,
    x: O,
    y: I,
    end_x: O,
    lower: bool,
}

pub type Point<T> = (T, T);

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl XiaolinWu<float, int> {
    #[inline]
    pub fn new(mut start: Point<float>, mut end: Point<float>) -> Self {
        let steep = (end.1 - start.1).abs() > (end.0 - start.0).abs();

        if steep {
            start = (start.1, start.0);
            end = (end.1, end.0);
        }

        if start.0 > end.0 {
            swap(&mut start, &mut end);
        }
        // log!("Line xs = {} - {}", start.0, end.0);

        let mut gradient = (end.1 - start.1) / (end.0 - start.0);

        if gradient == 0.0 {
            gradient = 1.0;
        }

        Self {
            steep,
            gradient,
            x: start.0.round() as int,
            y: start.1,
            end_x: end.0.round() as int,
            lower: false,
        }
    }
}

impl XiaolinWu<float, int> {
    // #[inline]
    // pub fn draw(&mut self, data: &mut [u8], width: usize, height: usize, r: u8, g: u8, b: u8) {
    //     for ((x, y), amount) in self {
    //         if x < 0 || y < 0 || x >= width as int || y >= height as int {
    //             continue;
    //         }
    //         let index = ((y as usize) * width + x as usize) * 4;
    //         let brightness = (amount * 255.0) as u8;
    //         data[index] = r;
    //         data[index + 1] = g;
    //         data[index + 2] = b;
    //         data[index + 3] = brightness;
    //     }
    // }

    #[inline]
    pub fn draw_brightness(&mut self, data: &mut [usize], width: usize, height: usize, full: float) {
        loop {
            let ((x, y), amount) = if self.x <= self.end_x {
                // get the fractional part of y
                let fpart = self.y - self.y.floor();

                // Calculate the integer value of y
                let mut y = NumCast::from(self.y).unwrap();
                if self.lower {
                    y += 1;
                }

                // Get the point
                let point = if self.steep { (y, self.x) } else { (self.x, y) };

                if self.lower {
                    // Return the lower point
                    self.lower = false;
                    self.x += 1;
                    self.y += self.gradient;
                    (point, fpart)
                } else {
                    if fpart > 0.0 {
                        // Set to return the lower point if the fractional part is > 0
                        self.lower = true;
                    } else {
                        // Otherwise move on
                        self.x += 1;
                        self.y += self.gradient;
                    }

                    // Return the remainer of the fractional part
                    (point, 1.0 - fpart)
                }
            } else {
                // log!("Bailing {}, {}", self.x, self.end_x);
                break;
            };
            if x < 0 || y < 0 || x >= width as int || y >= height as int {
                continue;
            }
            let index = (y as usize) * width + x as usize;
            let brightness = (amount * full) as usize;
            data[index] += brightness;
        }

        // :: this is the slower way

        // for ((x, y), amount) in self {
        //     if x < 0 || y < 0 || x >= width as int || y >= height as int {
        //         continue;
        //     }
        //     let index = (y as usize) * width + x as usize;
        //     let brightness = (amount * full) as usize;
        //     data[index] += brightness;
        // }
    }
}

// impl Iterator for XiaolinWu<float, int> {
//     type Item = (Point<int>, float);

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.x <= self.end_x {
//             // get the fractional part of y
//             let fpart = self.y - self.y.floor();

//             // Calculate the integer value of y
//             let mut y = NumCast::from(self.y).unwrap();
//             if self.lower {
//                 y += 1;
//             }

//             // Get the point
//             let point = if self.steep { (y, self.x) } else { (self.x, y) };

//             if self.lower {
//                 // Return the lower point
//                 self.lower = false;
//                 self.x += 1;
//                 self.y += self.gradient;
//                 Some((point, fpart))
//             } else {
//                 if fpart > 0.0 {
//                     // Set to return the lower point if the fractional part is > 0
//                     self.lower = true;
//                 } else {
//                     // Otherwise move on
//                     self.x += 1;
//                     self.y += self.gradient;
//                 }

//                 // Return the remainer of the fractional part
//                 Some((point, 1.0 - fpart))
//             }
//         } else {
//             // log!("Bailing {}, {}", self.x, self.end_x);
//             None
//         }
//     }
// }
