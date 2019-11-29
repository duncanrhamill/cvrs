
/// Trait used to limit data types for images
pub trait ImageDataType: Clone + From<u8> + Copy {}

// Types supported for the image data are u8, u16, f32, and f64
impl ImageDataType for u8 {}
impl ImageDataType for u16 {}
impl ImageDataType for f32 {}
impl ImageDataType for f64 {}

/// Basic representation of an image.
pub struct Image<T> where
    T: ImageDataType {

    /// Number of columns in the image
    cols: u32,

    /// Number of rows in the image
    rows: u32,

    /// The color depth of the image
    depth: u8,

    /// The color type of the image
    color_type: ColorType,

    /// Vector of image data. Images are stored as a vector in memory to be as 
    /// flexible as possible. 
    /// 
    /// The vector is organised as a flat version of a matrix. To access color 
    /// channel c for the pixel at (x, y) (with indexing starting at zero being
    /// the top left pixel), you would access element 
    /// data[depth * (y * cols + x) + c].
    data: Vec<T>
}

#[derive(Debug, PartialEq)]
pub enum ColorType {
    Mono,
    RGB,
    RGBA
}

impl std::fmt::Display for ColorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorType::Mono => write!(f, "Monochromatic"),
            ColorType::RGB  => write!(f, "RGB"),
            ColorType::RGBA => write!(f, "RGBA")
        }
    }
}

impl<T> Image<T> where
    T: ImageDataType {

    pub fn new(cols: u32, rows: u32, color_type: ColorType) -> Image<T> {
        match color_type {
            ColorType::Mono =>
                Image::<T> {
                    cols: cols,
                    rows: rows,
                    depth: 1 as u8,
                    color_type: color_type,
                    data: vec![T::from(0 as u8); (cols * rows) as usize]
                },
            ColorType::RGB =>
                Image::<T> {
                    cols: cols,
                    rows: rows,
                    depth: 3 as u8,
                    color_type: color_type,
                    data: vec![T::from(0 as u8); (cols * rows * 3) as usize]
                },
            ColorType::RGBA =>
                Image::<T> {
                    cols: cols,
                    rows: rows,
                    depth: 4 as u8,
                    color_type: color_type,
                    data: vec![T::from(0 as u8); (cols * rows * 4) as usize]
                }
        }
    }

    /// Get the data for a particular pixel
    /// 
    /// # Arguments
    /// *`col` - The zero-based column index for the pixel
    /// *`row` - The zero-based row index for the pixel
    /// 
    /// # Returns
    /// Vector of channels for the indexed pixel, or error if pixel is out of 
    /// bounds for the image. A vector is returned here to allow multi-channel
    /// data retrieval. For single channel (mono) using `get_pixel_mono` may be
    /// simpler.
    pub fn get_pixel_data(&self, col: u32, row: u32) -> Result<Vec<T>, String> {

        if (col < self.cols) && (row < self.rows) {

            let idx_bot 
                = (self.depth as u32 * (col * self.cols + row)) as usize;
            let idx_top = idx_bot +  self.depth as usize;

            let pix_data: Vec<T> 
                = self.data[idx_bot..idx_top].to_vec();

            Ok(pix_data)
        }
        else {
            Err(format!(
                "Out of bounds: Cannot get pixel data for ({}, {}) since the \
                image is only {}x{}.", col, row, self.cols, self.rows))
        }
    }

    /// Get monochromatic pixel information for the image 
    pub fn get_pixel_mono(&self, col: u32, row: u32) -> Result<T, String> {

        if self.color_type != ColorType::Mono {
            return Err(format!(
                "Cannot get monochromatic data for an image with a {} color \
                type.", self.color_type));
        }

        if (col < self.cols) && (row < self.rows) {
            let idx 
                = (col * self.cols + row) as usize;

            Ok(self.data[idx])
        }
        else {
            Err(format!(
                "Out of bounds: Cannot get pixel data for ({}, {}) since the \
                image is only {}x{}.", col, row, self.cols, self.rows))
        }
    }

    pub fn set_pixel_data(&mut self, col: u32, row: u32, val: Vec<T>) -> Result<(), String> {

        if (col < self.cols) && (row < self.rows) {

            let idx_bot 
                = (self.depth as u32 * (col * self.cols + row)) as usize;

            for i in 0..(self.depth as usize) {
                self.data[i + idx_bot] = val[i];
            }

            Ok(())
        }
        else {
            Err(format!(
                "Out of bounds: Cannot get pixel data for ({}, {}) since the \
                image is only {}x{}.", col, row, self.cols, self.rows))
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::image::*;

    #[test]
    fn image_init() {
        let img_mono: Image<u8> = Image::new(2, 2, ColorType::Mono);

        println!(
            "Created a new image with:\n  {} columns, {} rows, {} channels",
            img_mono.cols, img_mono.rows, img_mono.depth);

        println!("  Data: {:?}", img_mono.data);

        let img_rgb: Image<u8> = Image::new(2, 2, ColorType::RGB);

        println!(
            "Created a new image with:\n  {} columns, {} rows, {} channels",
            img_rgb.cols, img_rgb.rows, img_rgb.depth);

        println!("  Data: {:?}", img_rgb.data);

        println!("{:?}", img_mono.get_pixel_data(1, 0).unwrap());
        println!("{:?}", img_rgb.get_pixel_data(1, 0).unwrap());
        println!("{}", img_mono.get_pixel_mono(1, 0).unwrap());
        println!("{}", img_rgb.get_pixel_mono(1, 0).unwrap());
    }
}