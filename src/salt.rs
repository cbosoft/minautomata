use super::particle::{Particle, Neighbours};
use super::actions::Action;
use super::kinds::ParticleKind;
use super::colour::{Colour, WHITE};
use super::game::log;

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