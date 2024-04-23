
use bresenham::Bresenham;
use error::DrawError;
mod error;

pub struct GeometryDrawer {
    window_width: usize,
    window_height: usize,
}

impl GeometryDrawer {
    pub fn new(window_width: usize,window_height: usize) -> Self{
        Self { window_width, window_height }
    }

    pub fn draw_box(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        color: usize,
    ) -> Result<bool, DrawError>{
        for i in start_x..end_x {
            for j in start_y..end_y {
                if buf.len() < (j * self.window_width + i) {
                    return Err(DrawError::OutOfBounds(format!("x: {i} y: {j}").to_string()));
                } else {
                    buf[j * self.window_width + i] = color as u32;
                }
            }
        }
        Ok(true)
    }

    pub fn screen_clear(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
    ) -> Result<bool, DrawError>{
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
    ) -> Result<bool, DrawError>{
        for (x, y) in Bresenham::new(
            (start_x as isize, start_y as isize),
            (end_x as isize, end_y as isize),
        ) {
            if (y as usize * self.window_width + x as usize) < buf.len(){
                buf[y as usize * self.window_width + x as usize] = color as u32;
            }
            else {
                return Err(DrawError::OutOfBounds(format!("x: {x} y: {y}").to_string()));
            }
        }
        Ok(true)
    }
}









