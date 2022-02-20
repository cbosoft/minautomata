use super::particle::{Particle, Neighbours};
use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::{Colour, WHITE};

#[derive(Copy, Clone)]
pub struct SaltParticle;

impl Particle for SaltParticle {
    fn get_action(&self, neighbours: Neighbours) -> Action {
        if neighbours[1 + 1][0 + 1] == ParticleKind::Background {
            Action::MoveInto{x:0, y:1}
        }
        else {
            Action::StayPut
        }
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Salt
    }
    fn get_colour(&self) -> Colour {
        WHITE
    }
    fn tick(&self){

    }
}