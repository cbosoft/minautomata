
#[derive(Clone)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour{r:r, g:g, b:b}
    }

    pub fn as_uarr(&self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }

    pub fn as_i(&self) -> i32 {
        let r32 = self.r as i32;
        let g32 = self.g as i32;
        let b32 = self.b as i32;
        (r32 << 16) + (g32 << 8) + b32
    }

    pub fn i_eq(&self, i: i32) -> bool {
        i == self.as_i()
    }
}

// Colours
pub const BLACK: Colour = Colour{r: 0, g: 0, b: 0};
pub const DARK_GRAY: Colour = Colour{r: 64, g: 64, b: 64};
pub const GRAY: Colour = Colour{r: 128, g: 128, b: 128};
pub const LIGHT_GRAY: Colour = Colour{r: 196, g: 196, b: 196};
pub const WHITE: Colour = Colour{r: 255, g: 255, b: 255};

pub const RED: Colour = Colour{r: 255, g: 0, b: 0};
pub const GREEN: Colour = Colour{r: 0, g: 255, b: 0};
pub const BLUE: Colour = Colour{r: 0, g: 0, b: 255};


mod tests {

    use super::*;

    #[test]
    pub fn test_colours() {
        let red = Colour::new(255, 0, 0);
        assert_eq!(255 << 16, red.as_i());
    }

}