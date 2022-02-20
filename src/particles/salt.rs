use super::particle::{Particle, Neighbours, Processable};
use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::{Colour, WHITE};

#[derive(Copy, Clone, Default)]
pub struct SaltParticle{
    processed: bool
}

impl SaltParticle {

    pub fn new() -> SaltParticle {
        SaltParticle {
            processed: false
        }
    }

    fn is_solvent(k: ParticleKind) -> bool {
        k == ParticleKind::Water
    }

}

impl Particle for SaltParticle {
    fn get_action(&mut self, neighbours: Neighbours) -> Action {
        if Self::is_solvent(neighbours[0][0]) || Self::is_solvent(neighbours[0][1])
            || Self::is_solvent(neighbours[0][2]) || Self::is_solvent(neighbours[1][0])
            || Self::is_solvent(neighbours[1][2]) || Self::is_solvent(neighbours[2][0])
            || Self::is_solvent(neighbours[2][1]) || Self::is_solvent(neighbours[2][2]) {
                Action::Pop
        }
        else if neighbours[1 + 1][0 + 1] == ParticleKind::Background {
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
    fn tick(&mut self){
        self.set_not_processed();
    }
}

impl Processable for SaltParticle {
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