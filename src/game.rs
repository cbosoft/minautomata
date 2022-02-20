use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

use super::colour::*;
use super::kinds::ParticleKind;
use super::particle::{Particle, Neighbours};
use super::background::Background;
use super::salt::SaltParticle;
use super::concrete::ConcreteParticle;
use super::water::WaterParticle;
use super::actions::Action;
use super::point::Point;
use super::log::log;


const CANVAS_SIZE: usize = 128;
const MENU_HEIGHT: usize = 24;
const BASE_BUFFER_SIZE: usize = CANVAS_SIZE * CANVAS_SIZE;
const OUTPUT_BUFFER_SIZE: usize = BASE_BUFFER_SIZE * 4;


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

    fn get_cell_of_kind(kind: ParticleKind) -> Rc<dyn Particle> {
        match kind {
            ParticleKind::Background => Rc::new(Background),
            ParticleKind::Salt => Rc::new(SaltParticle),
            ParticleKind::Concrete => Rc::new(ConcreteParticle),
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
                        self.move_to(idx, (y + dy) as usize * CANVAS_SIZE + (x + dx) as usize);
                    },
                    Action::GrowInto{x: dx, y: dy, kind} => {
                        self.cells[(y + dy) as usize * CANVAS_SIZE + (x + dx) as usize] = Game::get_cell_of_kind(kind);
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
        

    }

    pub fn get_output_buffer_pointer(&self) -> *const u8 {
        let pointer: *const u8;
        pointer = self.output_buffer.as_ptr();
        return pointer;
    }
    
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


#[allow(dead_code,unused_imports)]
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

}