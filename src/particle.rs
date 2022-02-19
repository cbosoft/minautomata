use super::actions::Action;
use super::kinds::ParticleKind;
use super::colour::Colour;

pub type Neighbours = [[ParticleKind; 3]; 3];

pub trait Particle {
    fn get_action(&self, neighbours: Neighbours) -> Action;
    fn get_type(&self) -> ParticleKind;
    fn get_colour(&self) -> Colour;
    fn tick(&self);
}