use bresenham::Bresenham;
use error::DrawError;
mod error;

pub struct GeometryDrawer {
    window_width: usize,
    // window_height: usize, // Meant for future use
}

impl GeometryDrawer {
    pub fn new(window_width: usize,window_height: usize) -> Self{
        Self { window_width }
    }

    pub fn draw_box(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        color: usize,
    ) -> Result<(), DrawError>{
        for i in start_x..end_x {
            for j in start_y..end_y {
                self.draw_pixel(buf, i, j, color)?;
            }
        }
        Ok(())
    }

    pub fn screen_clear(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> Result<(), DrawError>{
        self.draw_box( buf, start_x, start_y, end_x, end_y, 0x00_00_00)
    }

    pub fn draw_line(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        color: usize,
    ) -> Result<(), DrawError>{
        for (x, y) in Bresenham::new(
            (start_x as isize, start_y as isize),
            (end_x as isize, end_y as isize),
        ) {
            self.draw_pixel(buf, x as usize, y as usize, color)?;
        }
        Ok(())
    }

    pub fn draw_rectangle(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        border_thickness: usize,
        color: usize,
    ) -> Result<(), DrawError>{
        for i in 0..border_thickness{
            self.draw_line(buf, start_x + i, start_y, start_x + i, end_y, color)?; 
            self.draw_line(buf, end_x - i -1, start_y, end_x - i -1, end_y, color)?;                  
            self.draw_line(buf, start_x, start_y + i, end_x, start_y + i, color)?;  
            self.draw_line(buf, start_x, end_y - i - 1, end_x, end_y - i - 1, color)?;
        }
        Ok(())
    }
    pub fn draw_circle(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        radius: usize,
        color: usize,
    ) -> Result<(), DrawError>{
        let mut d: isize = 3 - 2 * radius as isize;
        let mut x  = 0;
        let mut y = radius;
        while y >= x {
            x += 1;
            if d>0 {
                y -= 1;
                d = d + 4 * (x as isize - y as isize)  + 10;
            }
            else {
                d = d + 4 * x as isize + 6;
            }
            self.draw_pixel(buf, start_x + x, start_y + y, color)?;
            self.draw_pixel(buf, start_x - x, start_y + y, color)?;
            self.draw_pixel(buf, start_x + x, start_y - y, color)?;
            self.draw_pixel(buf, start_x - x, start_y - y, color)?;
            self.draw_pixel(buf, start_x + y, start_y + x, color)?;
            self.draw_pixel(buf, start_x - y, start_y + x, color)?;
            self.draw_pixel(buf, start_x + y, start_y - x, color)?;
            self.draw_pixel(buf, start_x - y, start_y - x, color)?;
        }
        Ok(())
    }

    fn draw_pixel(
        &self,
        buf: &mut Vec<u32>,
        x: usize,
        y: usize,
        color: usize
    ) -> Result<(), DrawError>{
        if (y * self.window_width + x ) < buf.len(){
            buf[y  * self.window_width + x ] = color as u32;
        }
        else {
            return Err(DrawError::OutOfBounds(format!("x: {x} y: {y}").to_string()));
        }
        Ok(())
    }

    
}









