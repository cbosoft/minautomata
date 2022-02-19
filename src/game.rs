use std::collections::HashMap;
use super::colour::*;
use super::kinds::ParticleKind;
use super::particle::{Particle, Neighbours};
use super::background::Background;
use super::salt::SaltParticle;
use super::concrete::ConcreteParticle;
use super::water::WaterParticle;
use super::actions::Action;
use std::rc::Rc;


#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);
}

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

// Define the size of our "checkerboard"
const CANVAS_SIZE: usize = 128;
const MENU_HEIGHT: usize = 24;
const BASE_BUFFER_SIZE: usize = CANVAS_SIZE * CANVAS_SIZE;
const OUTPUT_BUFFER_SIZE: usize = BASE_BUFFER_SIZE * 4;

#[derive(Clone, Hash, std::cmp::Eq)]
struct Point(usize, usize);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0) && (self.1 == other.1)
    }
}

#[wasm_bindgen]
pub struct Game {
    cells: Vec<Rc<dyn Particle>>,
    output_buffer: [u8; OUTPUT_BUFFER_SIZE],
    is_processed: [bool; BASE_BUFFER_SIZE],
    current_brush: ParticleKind,
    palette: HashMap<Point, ParticleKind>
}

#[wasm_bindgen]
impl Game {

    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let mut g = Game{
            cells: Vec::new(),
            output_buffer: [0; OUTPUT_BUFFER_SIZE],
            is_processed: [false; BASE_BUFFER_SIZE],
            current_brush: ParticleKind::Salt,
            palette: HashMap::new()
        };

        for _ in 0..BASE_BUFFER_SIZE {
            g.cells.push(Rc::new(Background));
        }
        log("Init!".to_string());

        g.init_palette();
        g.init_canvas();
        g.init_menu();
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

        self.hline(CANVAS_SIZE - 1 - MENU_HEIGHT, &BLUE);
        for dy in 0..MENU_HEIGHT {
            self.hline(CANVAS_SIZE - 1 - dy, &GRAY);
        }

        let y = CANVAS_SIZE - MENU_HEIGHT - 1;
        for x in 0..CANVAS_SIZE {
            let idx = y*CANVAS_SIZE + x;
            self.cells[idx] = Rc::new(ConcreteParticle);
        }
    }

    fn init_palette(&mut self) {
        let mut x: usize = 10;
        let mut y: usize = CANVAS_SIZE - MENU_HEIGHT + 5;

        let kinds = vec!(
            ParticleKind::Background,
            ParticleKind::Salt,
            ParticleKind::Concrete,
            ParticleKind::Water
        );

        for colour in kinds {
            self.palette.insert(Point(x, y), colour);
            y += 7;

            if y >= CANVAS_SIZE {
                y = CANVAS_SIZE - MENU_HEIGHT + 5;
                x += 10;
            }

        }
    }

    fn init_menu(&mut self) {
        for (pt, kind) in self.palette.clone() {
            let x = pt.0;
            let y = pt.1;
            for dx in 0..5 {
                for dy in 0..5 {
                    self.putv(x + dx - 2, y + dy - 2, BLACK.as_uarr());
                }
            }
            let p = Game::get_cell_of_kind(kind);
            let colour = p.as_ref().get_colour();
            for dx in 0..3 {
                for dy in 0..3 {
                    self.putv(x + dx - 1, y + dy - 1, colour.as_uarr());
                }
            }
        }
    }

    // fn point_is_valid(&self, x: i32, y: i32) -> bool {
    //     (x >= 0) && (x < CANVAS_SIZE as i32) && (y >= 0) && (y < (CANVAS_SIZE - MENU_HEIGHT) as i32)
    // }

    // fn point_is_clear(&self, x: i32, y: i32) -> bool {
    //     self.point_is_valid(x, y) && (self.getiv(x as usize, y as usize) == BLACK.as_i())
    // }

    // fn move_like_solid(&mut self, x: i32, y: i32) {
    //     if self.point_is_clear(x, y + 1) {self.move_to(x, y, x, y + 1);}
    // }

    // fn move_like_powder(&mut self, x: i32, y: i32) {
    //     if self.point_is_clear(x, y + 1) {self.move_to(x, y, x, y+1);}
    //     else if self.point_is_clear(x + 1, y + 1) {self.move_to(x, y, x + 1, y + 1);}
    //     else if self.point_is_clear(x - 1, y + 1) {self.move_to(x, y, x - 1, y + 1);}
    // }

    // fn move_like_liquid(&mut self, x: i32, y: i32) {
    //     if self.point_is_clear(x, y + 1) {self.move_to(x, y, x, y+1);}
    //     //else if self.point_is_clear(x + 1, y + 1) {self.move_to(x, y, x + 1, y + 1);}
    //     //else if self.point_is_clear(x - 1, y + 1) {self.move_to(x, y, x - 1, y + 1);}
    //     else if self.point_is_clear(x + 1, y) {self.move_to(x, y, x + 1, y);}
    //     else if self.point_is_clear(x - 1, y) {self.move_to(x, y, x - 1, y);}
    // }

    // fn move_to(&mut self, x: i32, y: i32, x2: i32, y2: i32) {
    //     let v = self.getv(x as usize, y as usize);
    //     self.putv(x as usize, y as usize, BLACK.as_uarr());
    //     self.putv(x2 as usize, y2 as usize, v);
    //     self.is_processed[(y2 as usize) * CANVAS_SIZE + x2 as usize] = true;
    // }

    fn get_cell_of_kind(kind: ParticleKind) -> Rc<dyn Particle> {
        match kind {
            ParticleKind::Background => Rc::new(Background),
            ParticleKind::Salt => Rc::new(SaltParticle),
            ParticleKind::Concrete => Rc::new(ConcreteParticle),
            ParticleKind::Vegetation => Rc::new(Background), // TODO
            ParticleKind::Water => Rc::new(WaterParticle)
        }
    }

    fn move_to(&mut self, source_idx: usize, dest_idx: usize) {
        self.cells[dest_idx] = self.cells[source_idx].clone();
        self.cells[source_idx] = Rc::new(Background);
        self.is_processed[dest_idx] = true;
    }

    #[wasm_bindgen]
    pub fn get_canvas_size(&self) -> i32 {
        CANVAS_SIZE as i32
    }

    #[wasm_bindgen]
    pub fn clicked(&mut self, x: usize, y: usize) {
        if y > CANVAS_SIZE - MENU_HEIGHT {
            self.menu_clicked(x, y)
        }
        else {
            self.paint(x, y)
        }
    }

    fn paint(&mut self, x: usize, y: usize) {
        //self.putv(x, y, self.current_brush.as_uarr());
        let idx = y*CANVAS_SIZE + x;
        self.cells[idx] = Game::get_cell_of_kind(self.current_brush);
    }

    fn menu_clicked(&mut self, x: usize, y: usize) {
        for dx in 0..5 {
            for dy in 0..5 {
                let pt = Point(x + dx - 2, y + dy - 2);
                if let Some(c) = self.palette.get(&pt) {
                    self.current_brush = c.clone();
                    self.init_menu();
                    self.draw_selection(pt.0, pt.1);
                    return;
                }
            }
        }
    }

    fn draw_selection(&mut self, x: usize, y: usize) {
        for dx in [0, 4] {
            for dy in 0..5 {
                self.putv(x + dx - 2, y + dy - 2, WHITE.as_uarr());
            }
        }
        for dy in [0, 4] {
            for dx in 0..5 {
                self.putv(x + dx - 2, y + dy - 2, WHITE.as_uarr());
            }
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {

        for y in 1..(CANVAS_SIZE-1) {
            for x in 1..(CANVAS_SIZE-1) {
                let idx = y*CANVAS_SIZE + x;
                self.is_processed[idx] = false;
            }
        }

        for y in 0..(CANVAS_SIZE as i32 - 1) {
            for x in 0..(CANVAS_SIZE as i32 - 1) {
                let idx = y as usize * CANVAS_SIZE + x as usize;
                if self.is_processed[idx] {
                    continue;
                }

                let mut neighbours: Neighbours = [[ParticleKind::Background; 3]; 3];
                for dy in 0..3 {
                    if ((y + dy - 1) < 0) || ((y + dy - 1) as usize >= CANVAS_SIZE) {
                        continue;
                    }
                    for dx in 0..3 {
                        if ((x + dx - 1) < 0) || ((x + dx - 1) as usize >= CANVAS_SIZE) {
                            continue;
                        }
                        let nidx = (y + dy - 1) as usize * CANVAS_SIZE + (x + dx - 1) as usize;
                        neighbours[dy as usize][dx as usize] = self.cells[nidx].as_ref().get_type();
                    }
                }

                match self.cells[idx].as_ref().get_action(neighbours) {
                    Action::Become(_kind) => (/* TODO */),
                    Action::MoveInto{x: dx, y: dy} => {
                        self.move_to(idx, (y + dy) as usize * CANVAS_SIZE + (x + dx) as usize)
                    },
                    Action::Pop => self.cells[idx] = Rc::new(Background),
                    Action::StayPut => ()
                }
            }
        }

        for y in 0..(CANVAS_SIZE - MENU_HEIGHT) {
            for x in 0..CANVAS_SIZE {
                let idx = y*CANVAS_SIZE + x;
                let colour = self.cells[idx].as_ref().get_colour();
                let colour_vec = colour.as_uarr();
                for off in 0..3 {
                    self.output_buffer[idx*4 + off] = colour_vec[off];
                }
                self.output_buffer[idx*4 + 3] = colour_vec[3];
            }
        }

        // //self.swap_buffers();

        // for y in 0..(CANVAS_SIZE - MENU_HEIGHT) {
        //     for x in 0..CANVAS_SIZE {

        //         if self.is_processed[y*CANVAS_SIZE + x] {
        //             continue;
        //         }

        //         let v = self.getiv(x, y);
                
        //         if GRAY.i_eq(v) {
        //             // do nothing
        //         }
        //         else if BLUE.i_eq(v) {
        //             self.move_like_liquid(x as i32, y as i32);
        //         }
        //         else if DARK_GRAY.i_eq(v) {
        //             self.move_like_solid(x as i32, y as i32);
        //         }
        //         else if !(BLACK.i_eq(v)) {
        //             self.move_like_powder(x as i32, y as i32);
        //         }

        //     }
        // }
        

    }

    pub fn get_output_buffer_pointer(&self) -> *const u8 {
        let pointer: *const u8;
        pointer = self.output_buffer.as_ptr();
        return pointer;
    }

    // fn swap_buffers(&mut self) {
    //     for y in 0..CANVAS_SIZE {
    //         for x in 0..CANVAS_SIZE {
    //             let idx = (y*CANVAS_SIZE + x)*4;
    //             for off in 0..4 {
    //                 self.read_buffer[idx + off] = self.write_buffer[idx + off];
    //             }
    //         }
    //     }
    // }
    
    fn putv(&mut self, x: usize, y: usize, v: [u8; 4]) {
        let idx = (y*CANVAS_SIZE + x)*4;
        for off in 0..4 {
            self.output_buffer[idx + off] = v[off];
        }
    }
    
    fn getv(&self, x: usize, y: usize) -> [u8; 4] {
        let mut rv: [u8; 4] = [0; 4];
        let idx = (y*CANVAS_SIZE + x)*4;
        for off in 0..4 {
            rv[off] = self.output_buffer[idx + off];
        }
        rv
    }
    
    fn getiv(&self, x: usize, y: usize) -> i32 {
        let mut iarr: [i32; 4] = [0; 4];
        let idx = (y*CANVAS_SIZE + x)*4;
        for i in 0..4 {
            iarr[i] = self.output_buffer[idx + i] as i32;
        }
        (iarr[0] << 16) + (iarr[1] << 8) + iarr[2]
    }
}


// mod tests {

//     use super::*;

//     #[test]
//     pub fn test_init() {
//         let _ = Game::new();
//     }

//     #[test]
//     pub fn test_update() {
//         let mut g = Game::new();
//         for _ in 0..10 {
//             g.update();
//         }
//     }

//     #[test]
//     pub fn test_colours() {
//         let red = Colour::new(255, 0, 0);
//         assert_eq!(255 << 16, red.as_i());
//     }

// }