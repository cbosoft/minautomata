use super::particle::{Particle, Neighbours, Processable};
use crate::actions::Action;
use crate::kinds::ParticleKind;
use crate::colour::{Colour, GREEN};

#[derive(Copy, Clone)]
pub struct CornucopiaParticle {
    particle_kind_to_create: ParticleKind
}

struct Pt(usize, usize);

impl CornucopiaParticle {

    pub fn new() -> Self {
        CornucopiaParticle {
            particle_kind_to_create: ParticleKind::Background
        }
    }

    fn maybe_find_new_kind(&mut self, neighbours: &Neighbours) {
        if self.particle_kind_to_create == ParticleKind::Background {
            self.find_new_kind(neighbours);
        }
    }

    fn find_new_kind(&mut self, neighbours: &Neighbours) {
        for i in 0..3 {
            for j in 0..3 {
                if i == j && i == 1 {
                    continue
                }

                if neighbours[i][j] != ParticleKind::Background {
                    self.particle_kind_to_create = neighbours[i][j];
                    return ();
                }
            }
        }
    }

    fn find_free_neighbour(neighbours: &Neighbours) -> Option<Pt> {
        for i in 0..3 {
            for j in 0..3 {
                if i == j && i == 1 {
                    continue
                }

                if neighbours[i][j] == ParticleKind::Background {
                    return Some(Pt(i, j));
                }
            }
        }

        return None;
    }
}

impl Particle for CornucopiaParticle {
    fn get_action(&mut self, neighbours: Neighbours) -> Action {
        self.maybe_find_new_kind(&neighbours);

        if self.particle_kind_to_create != ParticleKind::Background {

            match Self::find_free_neighbour(&neighbours) {
                Some(Pt(x, y)) => Action::GrowInto{x: x as i32 - 1, y: y as i32 - 1, kind:self.particle_kind_to_create},
                None => Action::StayPut
            }
        }
        else {
            Action::StayPut
        }
    }
    fn get_type(&self) -> ParticleKind {
        ParticleKind::Cornucopia
    }
    fn get_colour(&self) -> Colour {
        GREEN
    }
    fn tick(&mut self){
        self.set_not_processed()
    }
}

impl Processable for CornucopiaParticle {
    fn get_was_processed(&self) -> bool {
        false
    }

    fn set_processed(&mut self) {
        // do nothing
    }

    fn set_not_processed(&mut self) {
        // do nothing
    }
}