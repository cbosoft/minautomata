use super::particle::{Particle, Neighbours};
use super::actions::Action;
use super::kinds::ParticleKind;
use super::colour::{Colour, BLACK};

#[derive(Copy, Clone)]
pub struct Background;

impl Particle for Background {
    fn get_action(&self, neighbours: Neighbours) -> Action {
        Action::StayPut
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Background
    }
    fn get_colour(&self) -> Colour {
        BLACK
    }
    fn tick(&self){

    }
}