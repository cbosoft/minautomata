use super::colour::*;

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Define the size of our "checkerboard"
const CANVAS_SIZE: usize = 256;
const MENU_HEIGHT: usize = 48;
const OUTPUT_BUFFER_SIZE: usize = CANVAS_SIZE * CANVAS_SIZE * 4;


#[wasm_bindgen]
pub struct Game {
    read_buffer: [u8; OUTPUT_BUFFER_SIZE],
    write_buffer: [u8; OUTPUT_BUFFER_SIZE],
    current_brush: Colour
}

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let mut g = Game{
            read_buffer: [0; OUTPUT_BUFFER_SIZE],
            write_buffer: [0; OUTPUT_BUFFER_SIZE],
            current_brush: WHITE.clone()
        };

        g.init_canvas();
        g
    }

    fn hline(&mut self, y: usize, colour: &Colour) {
        for x in 0..CANVAS_SIZE {
            self.putv(x, y, colour.as_uarr());
        }
    }

    fn init_canvas(&mut self) {

        for y in 0..(CANVAS_SIZE - MENU_HEIGHT) {
            self.hline(y, &BLACK);
        }

        self.hline(CANVAS_SIZE - 1 - MENU_HEIGHT, &LIGHT_GRAY);
        for dy in 0..MENU_HEIGHT {
            self.hline(CANVAS_SIZE - 1 - dy, &GRAY);
        }
    }

    fn directly_below_is_clear(&self, x: usize, y: usize) -> bool {
        if y == CANVAS_SIZE - 1 - MENU_HEIGHT {
            false
        }
        else {
            self.getiv(x, y+1) == BLACK.as_i()
        }
    }

    fn move_directly_below(&mut self, x: usize, y: usize) {
        let v = self.getv(x, y);
        self.putv(x, y, BLACK.as_uarr());
        self.putv(x, y+1, v);
    }

    #[wasm_bindgen]
    pub fn get_canvas_size(&self) -> i32 {
        CANVAS_SIZE as i32
    }

    #[wasm_bindgen]
    pub fn clicked(&mut self, x: usize, y: usize) {
        self.putv(x, y, self.current_brush.as_uarr());
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {

        self.swap_buffers();

        for y in 0..CANVAS_SIZE {
            for x in 0..CANVAS_SIZE {

                let v = self.getiv(x, y);
                
                if WHITE.i_eq(v) {
                    if self.directly_below_is_clear(x, y) {
                        self.move_directly_below(x, y);
                    }
                }

            }
        }
        

    }

    pub fn get_output_buffer_pointer(&self) -> *const u8 {
        let pointer: *const u8;
        pointer = self.read_buffer.as_ptr();
        return pointer;
    }

    fn swap_buffers(&mut self) {
        for y in 0..CANVAS_SIZE {
            for x in 0..CANVAS_SIZE {
                let idx = (y*CANVAS_SIZE + x)*4;
                for off in 0..4 {
                    self.read_buffer[idx + off] = self.write_buffer[idx + off];
                }
            }
        }
    }
    
    fn putv(&mut self, x: usize, y: usize, v: [u8; 4]) {
        let idx = (y*CANVAS_SIZE + x)*4;
        for off in 0..4 {
            self.write_buffer[idx + off] = v[off];
        }
    }
    
    fn getv(&self, x: usize, y: usize) -> [u8; 4] {
        let mut rv: [u8; 4] = [0; 4];
        let idx = (y*CANVAS_SIZE + x)*4;
        for off in 0..4 {
            rv[off] = self.read_buffer[idx + off];
        }
        rv
    }
    
    fn getiv(&self, x: usize, y: usize) -> i32 {
        let mut iarr: [i32; 4] = [0; 4];
        let idx = (y*CANVAS_SIZE + x)*4;
        for i in 0..4 {
            iarr[i] = self.read_buffer[idx + i] as i32;
        }
        (iarr[0] << 16) + (iarr[1] << 8) + iarr[2]
    }
}


mod tests {

    use super::*;

    #[test]
    pub fn test_init() {
        let _ = Game::new();
    }

    #[test]
    pub fn test_update() {
        let mut g = Game::new();
        for _ in 0..10 {
            g.update();
        }
    }

    #[test]
    pub fn test_colours() {
        let red = Colour::new(255, 0, 0);
        assert_eq!(255 << 16, red.as_i());
    }

}