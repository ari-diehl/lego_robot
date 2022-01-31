use ev3dev_lang_rust::Ev3Result;

use super::paper::Paper;
use super::pen::Pen;

pub struct Plotter {
    pen: Pen,
    paper: Paper,
}

impl Plotter {
    pub fn new(pen: Pen, paper: Paper) -> Self {
        Self { pen, paper }
    }

    pub fn get_pen(&self) -> &Pen {
        &self.pen
    }

    pub fn get_paper(&self) -> &Paper {
        &self.paper
    }

    pub fn draw(&mut self, drawing_instructions: &Vec<Vec<(i32, i32)>>) -> Ev3Result<()> {
        self.pen.reset()?;

        self.paper.feed()?;

        for current_drawing_instructions in drawing_instructions {
            for (x_movement, y_movement) in current_drawing_instructions {
                self.pen.change_position(-*x_movement)?;
                self.paper.change_position(-*y_movement)?;

                self.pen.wait_until_not_moving();
                self.paper.wait_until_not_moving();

                if self.pen.get_is_lifted() {
                    self.pen.toggle_vertical_position()?;
                }
            }

            self.pen.toggle_vertical_position()?;
        }

        self.paper.remove()?;

        Ok(())
    }
}
