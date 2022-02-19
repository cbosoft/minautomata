// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Define the size of our "checkerboard"
const CANVAS_SIZE: usize = 20;
const OUTPUT_BUFFER_SIZE: usize = CANVAS_SIZE * CANVAS_SIZE * 4;
static mut READ_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];
static mut WRITE_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];
// Function to return a pointer to our buffer
// in wasm memory
#[wasm_bindgen]
pub fn get_output_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = READ_BUFFER.as_ptr();
    }

    return pointer;
}

unsafe fn swap_buffers() {
    for y in 0..CANVAS_SIZE {
        for x in 0..CANVAS_SIZE {
            let idx = (y*CANVAS_SIZE + x)*4;
            for off in 0..4 {
                READ_BUFFER[idx + off] = WRITE_BUFFER[idx + off];
            }
        }
    }
}

struct Colour {
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

// const RED: [u8; 4] = [255, 0, 0, 255];
// const GREEN: [u8; 4] = [0, 255, 0, 255];
// const BLUE: [u8; 4] = [0, 0, 255, 255];
const BLACK: Colour = Colour{r: 0, g: 0, b: 0};
const WHITE: Colour = Colour{r: 255, g: 255, b: 255};

fn putv(x: usize, y: usize, v: [u8; 4]) {
    let idx = (y*CANVAS_SIZE + x)*4;
    unsafe{
        for off in 0..4 {
            WRITE_BUFFER[idx + off] = v[off];
        }
    }
}

fn getv(x: usize, y: usize) -> [u8; 4] {
    let mut rv: [u8; 4] = [0; 4];
    let idx = (y*CANVAS_SIZE + x)*4;
    unsafe{
        for off in 0..4 {
            rv[off] = READ_BUFFER[idx + off];
        }
    }
    rv
}

fn getiv(x: usize, y: usize) -> i32 {
    let mut iarr: [i32; 4] = [0; 4];
    let idx = (y*CANVAS_SIZE + x)*4;
    unsafe{
        for i in 0..4 {
            iarr[i] = READ_BUFFER[idx + i] as i32;
        }
    }
    (iarr[0] << 16) + (iarr[1] << 8) + iarr[2]
}


#[wasm_bindgen]
pub fn init_game() {
    for y in 0..CANVAS_SIZE {
        for x in 0..CANVAS_SIZE {
            putv(x, y, BLACK.as_uarr());
        }
    }
    putv(CANVAS_SIZE/2, 2, WHITE.as_uarr());
}


#[wasm_bindgen]
pub fn update() {

    unsafe {
        swap_buffers();
    }

    for y in 0..CANVAS_SIZE {
        for x in 0..CANVAS_SIZE {

            let v = getiv(x, y);

            if WHITE.i_eq(v) { 
                // println!("({x}, {y}) => [{}, {}, {}]", v[0], v[1], v[2]);
                putv(x, y, BLACK.as_uarr());
                putv(x, if y < CANVAS_SIZE - 1 {y + 1} else {0}, WHITE.as_uarr());
                return ();
            }

        }
    }
    

}


mod tests {

    use super::*;

    #[test]
    pub fn test_init() {
        init_game();
    }

    #[test]
    pub fn test_update() {
        for _ in 0..10 {
            update();
        }
    }

    #[test]
    pub fn test_colours() {
        let red = Colour::new(255, 0, 0);
        assert_eq!(255 << 16, red.as_i());
    }

}