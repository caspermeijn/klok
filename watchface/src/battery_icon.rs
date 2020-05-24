use embedded_graphics::prelude::*;
use embedded_graphics::{egline, egrectangle, primitive_style};
use core::cmp::max;

pub struct BatteryIcon<C: PixelColor> {
    pub top_left: Point,
    pub bottom_right: Point,
    pub bg_color: C,
    pub fg_color: C,
    pub empty_color: C,
    pub full_color: C,
    pub state_of_charge: f32,
}

impl<C: PixelColor> Transform for BatteryIcon<C> {
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            bottom_right: self.bottom_right + by,
            ..*self
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;
        self.bottom_right += by;

        self
    }
}

impl<C: PixelColor> Dimensions for BatteryIcon<C> {
    fn top_left(&self) -> Point {
        self.top_left
    }

    fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    fn size(&self) -> Size {
        let width = (self.top_left.x - self.bottom_right.x).abs() as u32;
        let height = (self.top_left.y - self.bottom_right.y).abs() as u32;

        Size { width, height }
    }
}

impl<C: PixelColor> BatteryIcon<C> {
    fn border_color(&self) -> C {
        if self.state_of_charge >= 1.00 {
            self.full_color
        } else if self.state_of_charge < 0.10 {
            self.empty_color
        } else {
            self.fg_color
        }
    }

    fn contents_color(&self) -> C {
        if self.state_of_charge >= 1.00 {
            self.full_color
        } else if self.state_of_charge < 0.20 {
            self.empty_color
        } else {
            self.fg_color
        }
    }
    fn draw_bg<D: DrawTarget<C>>(&self, display: &mut D) -> Result<(), D::Error> {
        let style = primitive_style!(stroke_color = self.bg_color, fill_color = self.bg_color);
        egrectangle!(top_left = self.top_left,
            bottom_right = self.bottom_right,
            style = style)
            .draw(display)?;
        Ok(())
    }

    fn draw_border<D: DrawTarget<C>>(&self, display: &mut D) -> Result<(), D::Error> {

        //     6+------+5
        //      |      |
        //  8+--+7    4+--+3
        //   |            |
        //   |            |
        //   |            |
        //   |            |
        //  1+------------+2

        let height = self.size().height as i32;
        let width = self.size().width as i32;
        let point1 = Point::new(self.top_left.x, self.bottom_right.y);
        let point2 = self.bottom_right;
        let point3 = Point::new(self.bottom_right.x, self.top_left.y + height / 10 );
        let point4 = Point::new(self.bottom_right.x - width / 5, self.top_left.y + height / 10 );
        let point5 = Point::new(self.bottom_right.x - width / 5, self.top_left.y);
        let point6 = Point::new(self.top_left.x + width / 5, self.top_left.y);
        let point7 = Point::new(self.top_left.x + width / 5, self.top_left.y + height / 10);
        let point8 = Point::new(self.top_left.x, self.top_left.y + height / 10);

        let style = primitive_style!(
        stroke_color = self.border_color(),
        stroke_width = self.size().width / 10);
        egline!(start = point1, end = point2, style = style).draw(display)?;
        egline!(start = point2, end = point3, style = style).draw(display)?;
        egline!(start = point3, end = point4, style = style).draw(display)?;
        egline!(start = point4, end = point5, style = style).draw(display)?;
        egline!(start = point5, end = point6, style = style).draw(display)?;
        egline!(start = point6, end = point7, style = style).draw(display)?;
        egline!(start = point7, end = point8, style = style).draw(display)?;
        egline!(start = point8, end = point1, style = style).draw(display)?;

        Ok(())
    }

    fn draw_contents<D: DrawTarget<C>>(&self, display: &mut D) -> Result<(), D::Error> {

        //      +------+
        //      |rect1 |
        //   +--+------+--+
        //   |            |
        //   |   rect2    |
        //   |            |
        //   |            |
        //   +------------+

        let style = primitive_style!(
        stroke_color = self.contents_color(),
        fill_color = self.contents_color());

        let height = self.size().height as i32;
        let width = self.size().width as i32;

        let top_rect1 = self.top_left.y + height / 10;
        let bottom_rect1 = self.top_left.y + height / 10 + width / 5;
        let top_rect2 = bottom_rect1;
        let bottom_rect2 = self.bottom_right.y - width / 5;

        let content_height = bottom_rect2 - top_rect1;
        let max_top = bottom_rect2 - (content_height as f32 * self.state_of_charge) as i32;

        if max_top < bottom_rect1 {
            let left_rect1 = self.top_left.x + 2 * width / 5;
            let right_rect1 = self.bottom_right.x - 2 * width / 5;

            egrectangle!(top_left = Point::new(left_rect1, max(top_rect1, max_top)),
            bottom_right = Point::new(right_rect1, bottom_rect1),
            style = style)
                .draw(display)?;
        }

        if self.state_of_charge > 0.01 {
            let left_rect2 = self.top_left.x + width / 5;
            let right_rect2 = self.bottom_right.x - width / 5;

            egrectangle!(top_left = Point::new(left_rect2, max(top_rect2, max_top)),
            bottom_right = Point::new(right_rect2, bottom_rect2),
            style = style)
                .draw(display)?;
        }

        Ok(())
    }
}

impl<C> Drawable<C> for &BatteryIcon<C>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        self.draw_bg(display)?;
        self.draw_border(display)?;
        self.draw_contents(display)?;
        Ok(())
    }
}