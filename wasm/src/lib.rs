mod utils;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let hello = "Hello from wasm!";
    let message = format!("{} Your name is {}", hello, name);
    alert(&message);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dirty = 0,
    Clean = 1,
}

#[wasm_bindgen]
pub struct Grid {
    height: i32,
    width: i32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Grid {
    pub fn new(height: i32, width: i32) -> Grid {
        let len = usize::try_from(height * width).unwrap();
        let mut cells = Vec::with_capacity(len);

        for _n in 0..((height * width) as usize) {
            cells.push(Cell::Dirty);
        }

        Grid {
            height,
            width,
            cells
        }
    }
    
    pub fn get_cell(&self, row: i32, column: i32) -> Cell {
        self.cells[(row * self.width + column) as usize]
    }

    pub fn clean(&mut self, y: i32, x: i32, half_size: i32) -> i32 {
        let y_max = if y + half_size > self.height { self.height } else { y + half_size };
        let x_max = if x + half_size > self.width { self.width } else { x + half_size };

        for n in (y - half_size).abs()..y_max {
            for m in (x - half_size).abs()..x_max {
                self.cells[(n * self.width + m) as usize] = Cell::Clean;
            }
        }

        return x_max * y_max;
    }

    pub fn expand(&mut self, new_height: i32, new_width: i32) -> u8 {
        if new_height <= self.height && new_width <= self.width { 
            return 0u8;
        }

        let len = usize::try_from(new_height * new_width).unwrap();
        let mut cells = Vec::<Cell>::with_capacity(len);

        for _n in 0..new_height {
            for _m in 0..new_width {
                let index = usize::try_from(_n * self.width + _m).unwrap();
                let value: Cell = if _n >= self.height || _m >= self.width {
                    Cell::Dirty
                }
                else {
                    self.cells[index]
                };
                
                cells.push(value);
            }
        }
        self.width = new_width;
        self.height = new_height;
        self.cells = cells;

        return 1u8;
    }
    // ...
}