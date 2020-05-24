    use embedded_graphics::{
        pixelcolor::Rgb565,
        prelude::*,
    };
    use embedded_graphics_simulator::{SimulatorDisplay, Window, OutputSettingsBuilder, SimulatorEvent};

    use watchface::Watchface;
    use watchface::BatteryProvider;
    use watchface::TimeProvider;
    use std::thread;
    use std::time::Duration;
    use chrono::Timelike;

    struct NowProvider {}

    impl TimeProvider for NowProvider {
        fn get_time(&self) -> String {
            chrono::Local::now().format("%H:%M:%S").to_string()
        }
    }

    struct StubBatteryProvider {}

    impl BatteryProvider for StubBatteryProvider {
        fn get_state_of_charge(&self) -> f32 {
            chrono::Local::now().second() as f32 / 59.0
        }
    }

    fn main() -> Result<(), core::convert::Infallible> {
        let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(240, 240));

        let output_settings = OutputSettingsBuilder::new()
            .build();
        let mut window = Window::new("Klok watchface", &output_settings);

        let now_provider = NowProvider {};
        let battery_provider = StubBatteryProvider {};

        let watchface = Watchface::new(now_provider, battery_provider);

        'running: loop {
            watchface.draw(&mut display)?;
            window.update(&display);

            for event in window.events() {
                match event {
                    SimulatorEvent::Quit => break 'running,
                    _ => {}
                }
            }
            thread::sleep(Duration::from_millis(200));
        }

        Ok(())
    }
