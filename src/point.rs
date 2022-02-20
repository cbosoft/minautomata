#[derive(Clone, Hash, std::cmp::Eq)]
pub struct Point(pub usize, pub usize);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}