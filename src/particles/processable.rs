pub trait Processable {
    fn get_was_processed(&self) -> bool;
    fn set_processed(&mut self);
    fn set_not_processed(&mut self);
}