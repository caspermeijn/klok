use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

pub trait Center<D: DrawTarget<Rgb565>> {
    fn center(&mut self, display: &D) -> Self;
}

impl<D,T> Center<D> for T
    where D: DrawTarget<Rgb565>,
          T: embedded_graphics::transform::Transform + embedded_graphics::geometry::Dimensions,
{
    fn center(&mut self, display: &D) -> Self {
        let display_size = display.size();
        let drawable_size = self.size();
        let center_x = ((display_size.width - drawable_size.width) / 2) as i32;
        let center_y = ((display_size.height - drawable_size.height) / 2) as i32;
        let center_point = Point::new(center_x, center_y);
        self.translate(center_point)
    }
}

pub trait TopRight<D: DrawTarget<Rgb565>> {
    fn translate_to_top_right(&mut self, display: &D) -> Self;
}

impl<D,T> TopRight<D> for T
    where D: DrawTarget<Rgb565>,
          T: embedded_graphics::transform::Transform + embedded_graphics::geometry::Dimensions,
{
    fn translate_to_top_right(&mut self, display: &D) -> Self {
        let display_size = display.size();
        let drawable_size = self.size();
        let top_right_x = (display_size.width - drawable_size.width - 20) as i32;
        let top_right_y = 20;
        let top_right_point = Point::new(top_right_x, top_right_y);
        self.translate(top_right_point)
    }
}
