extern crate num_traits;
use num_traits::{Float, Signed, NumAssignOps, NumCast};
use std::mem::swap;

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
///     for ((x, y), value) in XiaolinWu::<f32, i16>::new((0.0, 0.0), (3.0, 6.0)) {
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

impl XiaolinWu<f32, i16> {
    #[inline]
    pub fn new(mut start: Point<f32>, mut end: Point<f32>) -> Self {
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
            x: start.0.round() as i16,
            y: start.1,
            end_x: end.0.round() as i16,
            lower: false,
        }
    }

}

impl XiaolinWu<f32, i16> {
    #[inline]
    pub fn draw(&mut self, data: &mut [u8], width: usize, height: usize, r: u8, g: u8, b: u8) {
        for ((x, y), amount) in self {
            if x < 0 || y < 0 || x >= width as i16 || y >= height as i16 {
                continue
            }
            let index = ((y as usize) * width + x as usize) * 4;
            let brightness = (amount * 255.0) as u8;
            data[index] = r;
            data[index + 1] = g;
            data[index + 2] = b;
            data[index + 3] = brightness;
        }
    }
}

impl Iterator for XiaolinWu<f32, i16> {
    type Item = (Point<i16>, f32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x <= self.end_x {
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
                Some((point, fpart))
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
                Some((point, 1.0 - fpart))
            }
        } else {
            // log!("Bailing {}, {}", self.x, self.end_x);
            None
        }
    }
}