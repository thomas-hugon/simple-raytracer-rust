use std::io::Write;
use crate::Color;

pub struct Ppm<T>{
    writer: T,
    width: u32,
    height: u32,
    colors: f64,
    current_c: u32,
    current_l: u32,
}

impl<T: Write> Ppm<T>{
    pub fn new(writer: T, width: u32, height: u32, colors: u32) -> Result<Self,std::io::Error> {
        let mut ppm = Ppm {
            writer,
            width,
            height,
            colors: colors as f64 - 0.01,
            current_c: 0,
            current_l: 0
        };
        ppm.writer.write_all(format!("P3\n{} {}\n{}\n", width, height, (colors-1)).as_bytes())?;
        Ok(ppm)
    }

    pub fn next_pixel(&mut self, color: Color) -> Result<(), std::io::Error>{
        if self.current_l >= self.height{
            return Ok(());
        }
        let red = (color.0 * self.colors) as u32;
        let green = (color.1 * self.colors) as u32;
        let blue = (color.2 * self.colors) as u32;
        if self.current_c < self.width - 1 {
            self.writer.write_all(format!("{} {} {} ", red, green, blue).as_bytes() )?;
            self.current_c += 1;
        }else{
            self.writer.write_all(format!("{} {} {}\n", red, green, blue).as_bytes() )?;
            self.current_c = 0;
            self.current_l += 1;
        }
        Ok(())
    }
}
