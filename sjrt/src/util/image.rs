use crate::{Color, IBuffer, IColorBuffer};

pub struct ImageBuffer {
    _width: i32,
    _height: i32,
    _image_buffer: ::image::RgbImage,
}

impl ImageBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            _width: width,
            _height: height,
            _image_buffer: ::image::ImageBuffer::new(width as u32, height as u32),
        }
    }

    pub fn save<TPath: std::convert::AsRef<std::path::Path>>(&self, path: TPath) {
        self._image_buffer.save(path).unwrap();
    }

    pub fn get_red(&self, x: i32, y: i32) -> u8 {
        let pixel = self._image_buffer.get_pixel(x as u32, y as u32);
        pixel[0]
    }

    pub fn get_green(&self, x: i32, y: i32) -> u8 {
        let pixel = self._image_buffer.get_pixel(x as u32, y as u32);
        pixel[1]
    }

    pub fn get_blue(&self, x: i32, y: i32) -> u8 {
        let pixel = self._image_buffer.get_pixel(x as u32, y as u32);
        pixel[2]
    }
}

impl IBuffer for ImageBuffer {
    fn get_width(&self) -> i32 {
        self._width
    }

    fn get_height(&self) -> i32 {
        self._height
    }

    fn set_color(&mut self, x: i32, y: i32, red: u8, green: u8, blue: u8) {
        let pixel = self._image_buffer.get_pixel_mut(x as u32, y as u32);
        *pixel = image::Rgb([red, green, blue]);
    }
}

impl IColorBuffer for &mut ImageBuffer {
    type Color = Color;

    fn write(&mut self, x: u32, y: u32, color: crate::Color) {
        let data = match color {
            crate::Color::R8G8B8A8_Uint(data) => [data[0], data[1], data[2]],
            crate::Color::R32G32B32A32_Unorm(data) => [
                (data[0] * 255.0).clamp(0.0, 255.0) as u8,
                (data[1] * 255.0).clamp(0.0, 255.0) as u8,
                (data[2] * 255.0).clamp(0.0, 255.0) as u8,
            ],
        };

        let rgb = image::Rgb(data);
        self._image_buffer.put_pixel(x, y, rgb);
    }
}
