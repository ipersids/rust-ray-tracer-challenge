//! # Canvas Module
//!
//! This module provides the `Canvas` struct, which represents a 2D grid of pixels.
//! Each pixel is represented by a `Color`, and the canvas can be used to render
//! images for the ray tracer challenge. The module also includes functionality
//! to export the canvas to a PPM (Portable Pixmap) file format.
//!
//! ## Features
//! - Create a canvas with default black pixels or a custom color.
//! - Add or get pixels at specific coordinates.
//! - Export the canvas to a PPM file.

use crate::Color;
use std::{fs, io::Write, path::Path};

const RENDERS_PATH: &str = "./renders";
const RENDERS_LINK: &str = "https://github.com/ipersids/rust-ray-tracer-challenge.git";
const RENDER_COLUMN_MAX: usize = 70;

/// A 2D grid of pixels used for rendering images.
///
/// ## Fields
/// - `width`: The width of the canvas in pixels.
/// - `height`: The height of the canvas in pixels.
/// - `pixels`: A vector storing the color of each pixel in row-major order.
#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    /// Creates a new canvas with the specified width and height.
    /// All pixels are initialized to black.
    pub fn new(width: usize, height: usize) -> Self {
        let capacity: usize = width * height;
        if capacity == 0 {
            eprintln!("Warning: pixels vector capacity equal zero.")
        }
        Self {
            width,
            height,
            pixels: vec![Color::new_black(); capacity],
        }
    }

    /// Creates a new canvas with the specified width, height and color.
    pub fn new_with_color(width: usize, height: usize, color: Color) -> Self {
        let capacity: usize = width * height;
        if capacity == 0 {
            eprintln!("Warning: pixels vector capacity equal zero.")
        }
        Self {
            width,
            height,
            pixels: vec![color; capacity],
        }
    }
}

impl Canvas {
    /// Sets the color of a pixel at the specified coordinates.
    pub fn add_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(
            x < self.width,
            "x coordinate {} out of bounds (width: {})",
            x,
            self.width
        );
        assert!(
            y < self.height,
            "y coordinate {} out of bounds (height: {})",
            y,
            self.height
        );
        let index = self.width * y + x;
        self.pixels[index] = color;
    }

    /// Returns the color of a pixel at the specified coordinates.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<Color> {
        match self.get_index(x, y) {
            Ok(i) => Some(self.pixels[i]),
            Err(_) => None,
        }
    }

    /// Calculates the index of a pixel using the specified coordinates.
    pub fn get_index(&self, x: usize, y: usize) -> Result<usize, &'static str> {
        if self.pixels.capacity() == 0 {
            return Err("Error: pixels vector capacity equal zero.");
        }
        if x >= self.width || y >= self.height {
            return Err("Error: coordinate is out of bounds.");
        }
        Ok(self.width * y + x)
    }
}

impl Canvas {
    /// Exports the canvas to a PPM (Portable Pixmap) file.
    ///
    /// ## Parameters
    /// - `filename`: An optional filename for the output file. If not provided,
    ///   the default name "test.ppm" will be used.
    ///
    /// ## Returns
    /// A `Result` indicating success or failure.
    pub fn to_ppm(&self, filename: Option<String>) -> Result<(), std::io::Error> {
        let name = filename.unwrap_or("test.ppm".to_string());
        let path = format!("{}/{}", RENDERS_PATH, name);

        if !Path::new(RENDERS_PATH).exists() {
            fs::create_dir_all(RENDERS_PATH)?;
            println!("Path '{}' successfully created.", RENDERS_PATH);
        }

        let mut f = fs::File::create(path)?;

        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.width, self.height)?;
        writeln!(f, "255")?;
        writeln!(f, "# source: {}", RENDERS_LINK)?;

        let mut items: Vec<String> = Vec::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let px = self.get_pixel(col, row).unwrap_or(Color::new_black());
                let red = px.get_clamped_red_u8();
                let green = px.get_clamped_green_u8();
                let blue = px.get_clamped_blue_u8();
                items.push(format!("{}", red));
                items.push(format!("{}", green));
                items.push(format!("{}", blue));
            }
        }

        let mut line_len: usize = 0;
        let mut counter: usize = 0;
        let max_count: usize = self.width * 3;
        for val in items.iter() {
            if counter >= max_count {
                writeln!(f)?;
                counter = 0;
                line_len = 0;
            }
            if line_len + val.len() + 1 > RENDER_COLUMN_MAX {
                writeln!(f)?;
                line_len = 0;
            }
            write!(f, "{} ", val)?;
            line_len += val.len() + 1;
            counter += 1;
        }

        println!("File '{}/{}' successfully saved.", RENDERS_PATH, name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _w: usize = 10;
        let _h: usize = 20;
        let _c: Color = Color::new_black();
        let res = Canvas::new(_w, _h);
        assert_eq!(res.width, _w);
        assert_eq!(res.height, _h);
        assert_eq!(res.pixels.capacity(), _w * _h);
        assert_eq!(res.pixels[0], _c);
    }

    #[test]
    fn test_get_index() {
        let _w: usize = 10;
        let _h: usize = 20;
        let res = Canvas::new(_w, _h);

        // Valid coordinates
        assert_eq!(res.get_index(0, 0), Ok(0));
        assert_eq!(res.get_index(1, 0), Ok(1));
        assert_eq!(res.get_index(0, 1), Ok(10));
        assert_eq!(res.get_index(9, 19), Ok(199));

        // Error x or y out-of-bounds
        assert_eq!(
            res.get_index(10, 0),
            Err("Error: coordinate is out of bounds.")
        );
    }

    #[test]
    fn test_canvas_custom_color() {
        let _w: usize = 3;
        let _h: usize = 3;
        let _c: Color = Color::new(0.2, 0.4, 0.6);
        let canvas = Canvas::new_with_color(_w, _h, _c);

        for y in 0.._h {
            for x in 0.._w {
                assert_eq!(canvas.get_pixel(x, y), Some(_c));
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_add_pixel_out_of_bounds() {
        let _w: usize = 5;
        let _h: usize = 5;
        let _c: Color = Color::new_white();
        let mut canvas = Canvas::new(_w, _h);
        canvas.add_pixel(6, 0, _c);
    }

    #[test]
    fn test_get_pixel_out_of_bounds() {
        let _w: usize = 5;
        let _h: usize = 5;
        let canvas = Canvas::new(_w, _h);

        assert_eq!(canvas.get_pixel(6, 0), None);
        assert_eq!(canvas.get_pixel(0, 6), None);
        assert_eq!(canvas.get_pixel(6, 6), None);
    }

    #[test]
    fn test_to_ppm() {
        let _w: usize = 5;
        let _h: usize = 3;
        let _c: Color = Color::new(1.0, 0.5, 0.25);
        let canvas = Canvas::new_with_color(_w, _h, _c);

        let filename = "test_output.ppm";
        let result = canvas.to_ppm(Some(filename.to_string()));
        assert!(result.is_ok());

        let path = format!("{}/{}", RENDERS_PATH, filename);
        assert!(Path::new(&path).exists());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_to_ppm_empty_canvas() {
        let canvas = Canvas::new(0, 0);

        let filename = "empty_canvas.ppm";
        let result = canvas.to_ppm(Some(filename.to_string()));
        assert!(result.is_ok());

        let path = format!("{}/{}", RENDERS_PATH, filename);
        assert!(Path::new(&path).exists());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_to_ppm_line_wrapping() {
        let _w: usize = 10;
        let _h: usize = 2;
        let _c: Color = Color::new(1.0, 0.8, 0.6);
        let canvas = Canvas::new_with_color(_w, _h, _c);

        let filename = "line_wrapping.ppm";
        let result = canvas.to_ppm(Some(filename.to_string()));
        assert!(result.is_ok());

        let path = format!("{}/{}", RENDERS_PATH, filename);
        assert!(Path::new(&path).exists());

        let ppm_content = fs::read_to_string(&path).unwrap();
        let lines: Vec<&str> = ppm_content.lines().collect();
        assert!(lines.iter().any(|line| line.len() <= RENDER_COLUMN_MAX));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_canvas_default_color() {
        let _w: usize = 4;
        let _h: usize = 4;
        let canvas = Canvas::new(_w, _h);

        for y in 0.._h {
            for x in 0.._w {
                assert_eq!(canvas.get_pixel(x, y), Some(Color::new_black()));
            }
        }
    }
}
