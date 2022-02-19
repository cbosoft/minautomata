use super::particle::{Particle, Neighbours};
use super::actions::Action;
use super::kinds::ParticleKind;
use super::colour::{Colour, LIGHT_GRAY};

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