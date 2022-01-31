use ev3dev_lang_rust::{
    motors::{LargeMotor, MediumMotor, MotorPort},
    sensors::{SensorPort, TouchSensor},
    Ev3Result,
};

use super::manual_control::ManualControl;

pub struct Pen {
    motor_horizontal: LargeMotor,
    motor_vertical: MediumMotor,
    touch_sensor: TouchSensor,
    is_lifted: bool,
}

impl Pen {
    pub fn new(
        motor_horizontal_port: MotorPort,
        motor_vertical_port: MotorPort,
        touch_sensor_port: SensorPort,
    ) -> Ev3Result<Self> {
        let motor_horizontal = LargeMotor::get(motor_horizontal_port)?;
        let motor_vertical = MediumMotor::get(motor_vertical_port)?;

        motor_horizontal.set_stop_action("brake")?;
        motor_horizontal.set_speed_sp(100)?;
        motor_vertical.set_stop_action("brake")?;
        motor_vertical.set_speed_sp(100)?;

        Ok(Self {
            motor_horizontal,
            motor_vertical,
            touch_sensor: TouchSensor::get(touch_sensor_port)?,
            is_lifted: true,
        })
    }

    pub fn get_is_lifted(&self) -> bool {
        self.is_lifted
    }

    pub fn reset(&self) -> Ev3Result<()> {
        if !self.touch_sensor.get_pressed_state()? {
            self.motor_horizontal.set_speed_sp(360)?;
            self.motor_horizontal.run_forever()?;
            self.motor_horizontal
                .wait(|| self.touch_sensor.get_pressed_state().unwrap(), None);
            self.motor_horizontal.stop()?;
        }

        self.motor_horizontal.run_to_rel_pos(Some(-100))?;
        self.motor_horizontal.wait_until_not_moving(None);

        self.motor_horizontal.set_speed_sp(100)?;

        Ok(())
    }

    pub fn toggle_vertical_position(&mut self) -> Ev3Result<()> {
        self.is_lifted = !self.is_lifted;

        self.motor_vertical.run_to_rel_pos(Some(180))?;
        self.motor_vertical.wait_until_not_moving(None);

        Ok(())
    }

    pub fn change_position(&self, movement_units: i32) -> Ev3Result<()> {
        self.motor_horizontal.run_to_rel_pos(Some(movement_units))?;

        Ok(())
    }

    pub fn wait_until_not_moving(&self) -> bool {
        self.motor_horizontal.wait_until_not_moving(None)
    }
}

pub enum PenControls {
    Left,
    Right,
    Up,
    Down,
    Stop,
}

impl ManualControl<PenControls> for Pen {
    fn controls(&self, command: PenControls) -> Ev3Result<()> {
        match command {
            PenControls::Left => {
                self.motor_horizontal.set_speed_sp(100)?;
                self.motor_horizontal.run_forever()?;
            }
            PenControls::Right => {
                self.motor_horizontal.set_speed_sp(-100)?;
                self.motor_horizontal.run_forever()?;
            }
            PenControls::Up => {
                self.motor_vertical.set_speed_sp(-100)?;
                self.motor_vertical.run_forever()?;
            }
            PenControls::Down => {
                self.motor_vertical.set_speed_sp(100)?;
                self.motor_vertical.run_forever()?;
            }
            PenControls::Stop => {
                self.motor_horizontal.stop()?;
                self.motor_vertical.stop()?;
            }
        }

        Ok(())
    }
}
