use super::particle::{Particle, Neighbours, Processable};
use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::{Colour, BLACK};

#[derive(Copy, Clone)]
pub struct Background;

impl Particle for Background {
    fn get_action(&mut self, neighbours: Neighbours) -> Action {
        Action::StayPut
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Background
    }
    fn get_colour(&self) -> Colour {
        BLACK
    }
    fn tick(&mut self){
        // do nothing
    }
}

impl Processable for Background {
    fn get_was_processed(&self) -> bool {
        true
    }

    fn set_processed(&mut self) {
        // do nothing
    }

    fn set_not_processed(&mut self) {
        // do nothing
    }
}