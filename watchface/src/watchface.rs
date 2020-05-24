use crate::battery_icon::*;
use crate::drawable_utils::*;
use embedded_graphics::{
    fonts::{Text, Font24x32},
    pixelcolor::Rgb565,
    prelude::*,
};
use embedded_graphics::style::TextStyleBuilder;

pub trait BatteryProvider {
    fn get_state_of_charge(&self) -> f32;
}

pub trait TimeProvider {
    fn get_time(&self) -> String;
}

pub struct Watchface<TP, BP>
    where TP: TimeProvider,
          BP: BatteryProvider
{
    time_provider: TP,
    battery_provider: BP,
}

impl<TP, BP> Watchface<TP, BP>
    where
        TP: TimeProvider,
        BP: BatteryProvider,
{
    pub fn new(time_provider: TP, battery_provider: BP) -> Watchface<TP, BP> {
        Watchface {
            time_provider,
            battery_provider,
        }
    }

    pub fn draw<D: DrawTarget<Rgb565>>(&self, display: &mut D) -> std::result::Result<(), D::Error> {

        let time_text_style = TextStyleBuilder::new(Font24x32)
            .text_color(Rgb565::BLUE)
            .background_color(Rgb565::BLACK)
            .build();

        let time = self.time_provider.get_time();
        Text::new(&time, Point::zero())
            .into_styled( time_text_style)
            .center(display)
            .draw(display)?;

        let battery_icon = BatteryIcon {
            top_left: Point::new(200,20),
            bottom_right: Point::new(210, 40),
            bg_color: Rgb565::BLACK,
            fg_color: Rgb565::WHITE,
            empty_color: Rgb565::RED,
            full_color: Rgb565::GREEN,
            state_of_charge: self.battery_provider.get_state_of_charge()
        };

        battery_icon.draw(display)?;

        Ok(())
    }
}