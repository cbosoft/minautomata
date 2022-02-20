use super::particle::{Particle, Neighbours};
use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::{Colour, LIGHT_GRAY};

#[derive(Copy, Clone)]
pub struct ConcreteParticle;

impl Particle for ConcreteParticle {
    fn get_action(&self, _: Neighbours) -> Action {
        Action::StayPut
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Concrete
    }
    fn get_colour(&self) -> Colour {
        LIGHT_GRAY
    }
    fn tick(&self){

    }
}