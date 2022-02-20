use super::particle::{Particle, Neighbours, Processable};
use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::{Colour, BLUE};

#[derive(Copy, Clone)]
pub struct WaterParticle {
    processed: bool,
    last_dir_x: i32
}

impl WaterParticle {

    pub fn new() -> WaterParticle {
        WaterParticle{
            processed: false,
            last_dir_x: 1
        }
    }

    fn is_empty(k: ParticleKind) -> bool {
        k == ParticleKind::Background
    }

}

impl Particle for WaterParticle {

    fn get_action(&mut self, neighbours: Neighbours) -> Action {

        let b_e = Self::is_empty(neighbours[2][1]);
        let ble = Self::is_empty(neighbours[2][0]);
        let bre = Self::is_empty(neighbours[2][2]);
        let jle = Self::is_empty(neighbours[1][0]);
        let jre = Self::is_empty(neighbours[1][2]);

        // if below is empty...
        if b_e { Action::MoveInto{x: 0, y: 1} }
        // if below right/left are empty...
        else if ble && bre { Action::MoveInto{x: self.last_dir_x, y: 1} }
        else if bre { self.last_dir_x = 1; Action::MoveInto{x: 1, y: 1} }
        else if ble { self.last_dir_x = -1; Action::MoveInto{x:-1, y: 1} }
        // if direct right/left are empty...
        else if jle && jre { Action::MoveInto{x: self.last_dir_x, y: 0} }
        else if neighbours[1][2] == ParticleKind::Background { self.last_dir_x = 1; Action::MoveInto{x: 1, y: 0} }
        else if neighbours[1][0] == ParticleKind::Background { self.last_dir_x = -1; Action::MoveInto{x:-1, y: 0} }
        // otherwise, do nothing
        else {Action::StayPut}
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Water
    }
    fn get_colour(&self) -> Colour {
        BLUE
    }
    fn tick(&mut self){
        self.set_not_processed();
    }
}

impl Processable for WaterParticle {
    fn get_was_processed(&self) -> bool {
        self.processed
    }

    fn set_processed(&mut self) {
        self.processed = true;
    }

    fn set_not_processed(&mut self) {
        self.processed = false
    }
}