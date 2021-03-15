use std::io::Write;
use crate::Color;

//fichier graphique de type bitmap textuel
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
        //entete
        ppm.writer.write_all(format!("P3\n{} {}\n{}\n", width, height, (colors-1)).as_bytes())?;
        Ok(ppm)
    }

    //un pixel est composé dans l'ordre de R, G, B séparés par des espaces.
    //Les pixels d'une même ligne sont séparés entre eux par des espaces.
    //on passe à la ligne suivante avec un newline \n
    pub fn next_pixel(&mut self, color: Color) -> Result<(), std::io::Error>{
        if self.current_l >= self.height{
            return Ok(());
        }
        let (red, green, blue) = color.scale(self.colors);
        if self.current_c < self.width - 1 {
            self.writer.write_all(format!("{} {} {} ", red, green, blue).as_bytes() )?;
            self.current_c += 1;
        }else{
            self.writer.write_all(format!("{} {} {}\n", red, green, blue).as_bytes() )?;
            self.current_c = 0;
            self.current_l += 1;
            println!("line {} printed", self.current_l);
        }
        Ok(())
    }
}
