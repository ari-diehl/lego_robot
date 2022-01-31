use ev3dev_lang_rust::{
    motors::{LargeMotor, MotorPort},
    sensors::{ColorSensor, Sensor, SensorPort},
    Ev3Result,
};

use super::manual_control::ManualControl;

pub struct Paper {
    motor: LargeMotor,
    color_sensor: ColorSensor,
}

impl Paper {
    pub fn new(motor_port: MotorPort, color_sensor_port: SensorPort) -> Ev3Result<Self> {
        let motor = LargeMotor::get(motor_port)?;
        let color_sensor = ColorSensor::get(color_sensor_port)?;

        motor.set_speed_sp(100)?;
        motor.set_stop_action("brake")?;
        color_sensor.set_mode_col_reflect()?;

        Ok(Self {
            motor,
            color_sensor,
        })
    }

    pub fn feed(&self) -> Ev3Result<()> {
        if self.color_sensor.get_value0()? < 10 {
            self.motor.set_speed_sp(-360)?;
            self.motor.run_forever()?;
            self.motor
                .wait(|| self.color_sensor.get_value0().unwrap() >= 10, None);
            self.motor.stop()?;

            self.motor.set_speed_sp(100)?;
        }

        Ok(())
    }

    pub fn remove(&self) -> Ev3Result<()> {
        if self.color_sensor.get_value0()? >= 10 {
            self.motor.set_speed_sp(-360)?;
            self.motor.run_forever()?;
            self.motor
                .wait(|| self.color_sensor.get_value0().unwrap() < 10, None);
            self.motor.stop()?;

            self.motor.set_speed_sp(100)?;
        }

        Ok(())
    }

    pub fn change_position(&self, movement_units: i32) -> Ev3Result<()> {
        self.motor.run_to_rel_pos(Some(movement_units))?;

        Ok(())
    }

    pub fn wait_until_not_moving(&self) -> bool {
        self.motor.wait_until_not_moving(None)
    }
}

pub enum PaperControls {
    In,
    Out,
    Stop,
}

impl ManualControl<PaperControls> for Paper {
    fn controls(&self, command: PaperControls) -> Ev3Result<()> {
        match command {
            PaperControls::In => {
                self.motor.set_speed_sp(-100)?;
                self.motor.run_forever()?;
            }
            PaperControls::Out => {
                self.motor.set_speed_sp(100)?;
                self.motor.run_forever()?;
            }
            PaperControls::Stop => self.motor.stop()?,
        }
        Ok(())
    }
}
