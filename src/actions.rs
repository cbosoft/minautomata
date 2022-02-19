use super::kinds::ParticleKind;


pub enum Action {
    Pop,
    StayPut,
    MoveInto{x: i32, y: i32},
    GrowInto{x: i32, y: i32},
    Become(ParticleKind)
}