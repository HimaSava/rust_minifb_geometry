use bresenham::Bresenham;
use error::DrawError;
mod error;


/// A struct representing a geometry drawer.
pub struct GeometryDrawer {
    window_width: usize,
}

impl GeometryDrawer {
    /// Creates a new `GeometryDrawer` instance.
    ///
    /// # Arguments
    ///
    /// * `window_width` - The width of the window.
    ///
    /// # Returns
    ///
    /// A new `GeometryDrawer` instance.
    pub fn new(window_width: usize) -> Self {
        Self { window_width }
    }

    /// Draws a box on the buffer.
    ///
    /// # Arguments
    ///
    /// * `buf` - The buffer to draw on.
    /// * `start_x` - The starting x-coordinate of the box.
    /// * `start_y` - The starting y-coordinate of the box.
    /// * `end_x` - The ending x-coordinate of the box.
    /// * `end_y` - The ending y-coordinate of the box.
    /// * `color` - The color of the box.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the box is successfully drawn, or a `DrawError` if an error occurs.
    pub fn draw_box(
        &self,
        buf: &mut Vec<u32>,
        start_x: usize,
        start_y: usize,
        end_x: usize,
        end_y: usize,
        color: usize,
    ) -> Result<(), DrawError> {
        for i in start_x..end_x {
            for j in start_y..end_y {
                self.draw_pixel(buf, i, j, color)?;
            }
        }
        Ok(())
    }

    /// Clears the screen.
    /// 
    /// # Arguments
    /// 
    /// * `buf` - The buffer to clear.
    /// * `start_x` - The starting x-coordinate of the screen.
    /// * `start_y` - The starting y-coordinate of the screen.
    /// * `end_x` - The ending x-coordinate of the screen.
    /// * `end_y` - The ending y-coordinate of the screen.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the screen is successfully cleared, or a `DrawError` if an error occurs.
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

    /// Draws a line on the buffer. 
    ///  
    /// # Arguments
    /// 
    /// * `buf` - The buffer to draw on.
    /// * `start_x` - The starting x-coordinate of the line.
    /// * `start_y` - The starting y-coordinate of the line.
    /// * `end_x` - The ending x-coordinate of the line.
    /// * `end_y` - The ending y-coordinate of the line.
    /// * `color` - The color of the line.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the line is successfully drawn, or a `DrawError` if an error occurs.
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

    /// Draws a rectangle on the buffer.
    /// 
    /// # Arguments
    /// 
    /// * `buf` - The buffer to draw on.
    /// * `start_x` - The starting x-coordinate of the rectangle.
    /// * `start_y` - The starting y-coordinate of the rectangle.
    /// * `end_x` - The ending x-coordinate of the rectangle.
    /// * `end_y` - The ending y-coordinate of the rectangle.           
    /// * `border_thickness` - The thickness of the border of the rectangle.
    /// * `color` - The color of the rectangle.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the rectangle is successfully drawn, or a `DrawError` if an error occurs.
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

    /// Draws a circle on the buffer.
    /// 
    /// # Arguments
    /// 
    /// * `buf` - The buffer to draw on.
    /// * `start_x` - The starting x-coordinate of the circle.
    /// * `start_y` - The starting y-coordinate of the circle.
    /// * `radius` - The radius of the circle.
    /// * `color` - The color of the circle.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the circle is successfully drawn, or a `DrawError` if an error occurs.
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

    /// Draws a pixel on the buffer.
    /// 
    /// # Arguments
    /// 
    /// * `buf` - The buffer to draw on.
    /// * `x` - The x-coordinate of the pixel.
    /// * `y` - The y-coordinate of the pixel.
    /// * `color` - The color of the pixel.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the pixel is successfully drawn, or a `DrawError` if an error occurs.
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









