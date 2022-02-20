use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::Colour;
pub use super::processable::Processable;

pub type Neighbours = [[ParticleKind; 3]; 3];

pub trait Particle : Processable {
    fn get_action(&mut self, neighbours: Neighbours) -> Action;
    fn get_type(&self) -> ParticleKind;
    fn get_colour(&self) -> Colour;
    fn tick(&mut self);
}