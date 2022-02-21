use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;

use super::colour::*;
use super::kinds::ParticleKind;
use super::particles::{
    particle::{Particle, Neighbours},
    background::Background,
    salt::SaltParticle,
    concrete::ConcreteParticle,
    water::WaterParticle,
    cornucopia::CornucopiaParticle
};
use super::actions::Action;
use super::point::Point;


const CANVAS_SIZE: usize = 128;
const MENU_HEIGHT: usize = 24;
const BASE_BUFFER_SIZE: usize = CANVAS_SIZE * CANVAS_SIZE;
const GAME_N_ROWS: usize = CANVAS_SIZE - MENU_HEIGHT;
const GAME_N_COLS: usize = CANVAS_SIZE;
const GAME_N_CELLS: usize = GAME_N_COLS * GAME_N_ROWS;
const OUTPUT_BUFFER_SIZE: usize = BASE_BUFFER_SIZE * 4;

/// A structure containing data and functions for running the game.
#[wasm_bindgen]
pub struct Game {
    cells: Vec<Rc<RefCell<dyn Particle>>>,
    output_buffer: [u8; OUTPUT_BUFFER_SIZE],
    current_brush: ParticleKind,
    palette: HashMap<Point, ParticleKind>
}

/// Functions for game
#[wasm_bindgen]
impl Game {

    /// create new game
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let mut g = Game{
            cells: Vec::new(),
            output_buffer: [0; OUTPUT_BUFFER_SIZE],
            current_brush: ParticleKind::Salt,
            palette: HashMap::new()
        };

        for _ in 0..GAME_N_CELLS {
            g.cells.push(Rc::new(RefCell::new(Background)));
        }

        g.init_palette();
        g.init_canvas();
        g.init_menu();
        g
    }

    fn hline(&mut self, y: usize, colour: &Colour) {
        for x in 0..GAME_N_COLS {
            self.putv(x, y, colour.as_uarr());
        }
    }

    fn init_canvas(&mut self) {

        for y in GAME_N_ROWS..CANVAS_SIZE {
            self.hline(y, &GRAY);
        }

        let y = GAME_N_ROWS - 1;
        for x in 0..GAME_N_COLS {
            let idx = y*GAME_N_COLS + x;
            self.cells[idx] = Game::get_cell_of_kind(ParticleKind::Concrete);
        }
    }

    fn init_palette(&mut self) {
        let mut x: usize = 10;
        let mut y: usize = GAME_N_ROWS + 5;

        let kinds = vec!(
            ParticleKind::Background,
            ParticleKind::Salt,
            ParticleKind::Concrete,
            ParticleKind::Water,
            ParticleKind::Cornucopia
        );

        for colour in kinds {
            self.palette.insert(Point(x, y), colour);
            y += 7;

            if y >= CANVAS_SIZE {
                y = GAME_N_ROWS + 5;
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
            let colour = p.borrow().get_colour();
            for dx in 0..3 {
                for dy in 0..3 {
                    self.putv(x + dx - 1, y + dy - 1, colour.as_uarr());
                }
            }
        }
    }

    fn get_cell_of_kind(kind: ParticleKind) -> Rc<RefCell<dyn Particle>> {
        match kind {
            ParticleKind::Background => Rc::new(RefCell::new(Background)),
            ParticleKind::Salt => Rc::new(RefCell::new(SaltParticle::new())),
            ParticleKind::Concrete => Rc::new(RefCell::new(ConcreteParticle)),
            ParticleKind::Water => Rc::new(RefCell::new(WaterParticle::new())),
            ParticleKind::Cornucopia => Rc::new(RefCell::new(CornucopiaParticle::new()))
        }
    }

    fn move_to(&mut self, source_idx: usize, dest_idx: usize) {
        if dest_idx < GAME_N_CELLS {
            self.cells[dest_idx] = self.cells[source_idx].clone();
            self.cells[dest_idx].borrow_mut().set_processed();
        }
        // else { /* out-of-bounds */ }

        self.cells[source_idx] = Game::get_cell_of_kind(ParticleKind::Background);
    }

    #[wasm_bindgen]
    pub fn get_canvas_size(&self) -> i32 {
        CANVAS_SIZE as i32
    }

    #[wasm_bindgen]
    pub fn clicked(&mut self, x: usize, y: usize) {
        if y >= GAME_N_ROWS {
            self.menu_clicked(x, y)
        }
        else {
            self.paint(x, y)
        }
    }

    fn paint(&mut self, x: usize, y: usize) {
        let idx = y*GAME_N_COLS + x;
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

        for y in 0..GAME_N_ROWS {
            for x in 0..GAME_N_COLS {
                let idx = y*CANVAS_SIZE + x;
                self.cells[idx].borrow_mut().tick();
            }
        }

        for y in 0..(GAME_N_ROWS as i32) {
            for x in 0..(GAME_N_COLS as i32) {
                let idx = y as usize * GAME_N_COLS + x as usize;
                if self.cells[idx].borrow().get_was_processed() {
                    // this particle has already interacted this turn, 
                    continue;
                }

                let mut neighbours: Neighbours = [[ParticleKind::Background; 3]; 3];
                for dy in 0..3 {
                    let ny = y + dy - 1;
                    if (ny < 0) || (ny as usize >= GAME_N_ROWS) {
                        continue;
                    }
                    for dx in 0..3 {
                        let nx = x + dx - 1;
                        if (nx < 0) || (nx as usize >= GAME_N_COLS) {
                            continue;
                        }
                        let nidx = ny as usize * GAME_N_COLS + nx as usize;
                        if nidx >= GAME_N_CELLS {
                            continue;
                        }
                        else {
                            neighbours[dy as usize][dx as usize] = self.cells[nidx].borrow().get_type();
                        }
                    }
                }

                let action = self.cells[idx].borrow_mut().get_action(neighbours);
                match action {
                    Action::Become(_kind) => (/* TODO */),
                    Action::MoveInto{x: dx, y: dy} => {
                        self.move_to(idx, (y + dy) as usize * GAME_N_COLS + (x + dx) as usize);
                    },
                    Action::GrowInto{x: dx, y: dy, kind} => {
                        self.cells[(y + dy) as usize * GAME_N_COLS + (x + dx) as usize] = Game::get_cell_of_kind(kind);
                    },
                    Action::Pop => self.cells[idx] = Game::get_cell_of_kind(ParticleKind::Background),
                    Action::StayPut => ()
                }
            }
        }

        for y in 0..GAME_N_ROWS {
            for x in 0..GAME_N_COLS {
                let idx = y*GAME_N_COLS + x;
                let colour = self.cells[idx].borrow().get_colour();
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
        let idx = (y*GAME_N_COLS + x)*4;
        for off in 0..4 {
            self.output_buffer[idx + off] = v[off];
        }
    }
    
    fn getv(&self, x: usize, y: usize) -> [u8; 4] {
        let mut rv: [u8; 4] = [0; 4];
        let idx = (y*GAME_N_COLS + x)*4;
        for off in 0..4 {
            rv[off] = self.output_buffer[idx + off];
        }
        rv
    }
    
    fn getiv(&self, x: usize, y: usize) -> i32 {
        let mut iarr: [i32; 4] = [0; 4];
        let idx = (y*GAME_N_COLS + x)*4;
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