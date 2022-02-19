use super::particle::{Particle, Neighbours};
use super::actions::Action;
use super::kinds::ParticleKind;
use super::colour::{Colour, BLUE};

#[derive(Copy, Clone)]
pub struct WaterParticle;

impl Particle for WaterParticle {
    fn get_action(&self, neighbours: Neighbours) -> Action {
        // if below is empty...
        if neighbours[2][1] == ParticleKind::Background { Action::MoveInto{x: 0, y: 1} }
        // if below right/left are empty
        else if neighbours[2][2] == ParticleKind::Background { Action::MoveInto{x: 1, y: 1} }
        else if neighbours[2][0] == ParticleKind::Background { Action::MoveInto{x:-1, y: 1} }
        // if direct right/left are empty
        else if neighbours[1][2] == ParticleKind::Background { Action::MoveInto{x: 1, y: 0} }
        else if neighbours[1][0] == ParticleKind::Background { Action::MoveInto{x:-1, y: 0} }
        // otherwise, do nothing
        else {Action::StayPut}
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Water
    }
    fn get_colour(&self) -> Colour {
        BLUE
    }
    fn tick(&self){

    }
}