mod utils;
extern crate js_sys;
// extern crate web_sys;


// use web_sys::console;

// pub struct Timer<'a> {
//     name: &'a str,
// }

// impl<'a> Timer<'a> {
//     pub fn new(name: &'a str) -> Timer<'a> {
//         console::time_with_label(name);
//         Timer { name }
//     }
// }

// impl<'a> Drop for Timer<'a> {
//     fn drop(&mut self) {
//         console::time_end_with_label(self.name);
//     }
// }


// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}



use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use web_sys::Window;




const DELTA:i32 = 1;
const COLOUR_THRESHOLDS :[i32;6]=[0xFFFF00,0x00FF00,0x00FFFF,0x0000FF,0xFF00FF,0xFF0000];
const COLOUR_DELTAS : [i32;6] =[
            0x000100 * DELTA,
            -0x010000 * DELTA,
            0x000001 * DELTA,
            -0x000100 * DELTA,
            0x010000 * DELTA,
            -0x000001 * DELTA];
const SIGNS : [i32;6] = [1,-1,1,-1,1,-1];
const NEXT_COLOUR : [i32;6] = [0xFFFF00,0x00FF00,0x00FFFF,0x0000FF,0xFF00FF,0xFF0000];


#[wasm_bindgen]  // Comment out for bench test
pub struct Universe {
    width_bits: usize,
    height_bits: usize,
    cells: Vec<u8>,
    fg_colour: i32,
    bg_colour: u32,
    colour_status: usize,
    fg_red: u8,
    fg_green: u8,
    fg_blue: u8,
    width: u32,
    height: u32,
}

const ALIVE: u8 = 1;
const DEAD: u8 = 0;

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) & (self.height - 1);
                let neighbor_col = (column + delta_col) & (self.width - 1);
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    

    fn increment_colour(& mut self) {

        

        match self.colour_status {
            0..=5 => { 
                self.fg_colour += COLOUR_DELTAS[self.colour_status];
                if  (self.fg_colour - COLOUR_THRESHOLDS[self.colour_status]) * SIGNS[self.colour_status] >=  0  {
                    self.fg_colour = NEXT_COLOUR[self.colour_status];
                    self.colour_status = (self.colour_status + 1) % 6;
                    
                    }
                },
            _ => {
                self.fg_colour = 0xFF0000;
                self.colour_status = 0;
                },
            };


        self.fg_red = ((self.fg_colour >> 16) & 0xFF) as u8;
        self.fg_green = ((self.fg_colour >> 8) & 0xFF) as u8;
        self.fg_blue = (self.fg_colour  & 0xFF) as u8;

        // let ten_millis = time::Duration::from_millis(10);

        // thread::sleep(ten_millis);

    }
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]  // Comment out for bench testt
impl Universe {

    
        

    pub fn tick(&mut self) {
        // let _timer = Timer::new("Universe::tick");

        let mut next = {
        //     let _timer = Timer::new("allocate next cells");
            self.cells.clone()
        };



        // let _timer = Timer::new("new generation");
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                // log!("cell[{}, {}] is initially {:?} and has {} live neighbors",
                //     row,
                //     col,
                //     cell,
                //     live_neighbors);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (ALIVE, x) if x < 2 => DEAD,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (ALIVE, 2) | (ALIVE, 3) => ALIVE,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (ALIVE, x) if x > 3 => DEAD,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (DEAD, 3) => ALIVE,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }
        // let _timer = Timer::new("free old cells");
        self.cells = next;
        // let _timer = Timer::new("colour change");
        self.increment_colour();
        self.cells[(self.height  * self.width) as usize] =  self.fg_red;
        self.cells[(self.height  * self.width+1) as usize] =  self.fg_green;
        self.cells[(self.height  * self.width+2) as usize] =  self.fg_blue;
        


    }
    
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width_bits = 8;
        let height_bits = 7;
        let fg_colour:i32 = 0xFF0000;
        let bg_colour: u32 = 0x00FF00;
        let colour_status = 0;
        let fg_red: u8 = 0xFF;
        let fg_green: u8 = 0;
        let fg_blue: u8 = 0;
        let width = 1 << width_bits;
        let height = 1 << height_bits;

        let cells: Vec<u8> = Universe::init_cells(width, height, fg_red, fg_green, fg_blue);

        // let cells: Vec<u8> = (0..width * height + 3)
        //     .map(|i| {
        //         if i < width * height {
        //             if i % 2 == 0 || i % 7 == 0 {
        //                 ALIVE
        //             } else {
        //                 DEAD
        //             }
        //         } else if i == width * height {
        //             fg_red
        //         } else if i == width * height + 1 {
        //             fg_green
        //         } else {
        //             fg_blue
        //         }
        //     })
        //     .collect();
        
        Universe {
            width_bits,
            height_bits,
            cells,
            fg_colour,
            bg_colour,
            colour_status,
            fg_red,
            fg_green,
            fg_blue,
            width,
            height,

        }

        
    }

    // pub fn render(&self) -> String {
    //     self.to_string()
    // }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u8 {
        self.cells.as_ptr()
    }
    
    pub fn fg_colour(&self) -> i32 {
        self.fg_colour
    }

    pub fn bg_colour(&self) -> u32 {
        self.bg_colour
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height + 3).map(|_i| DEAD).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height + 3).map(|_i| DEAD).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, col : u32) {
        let idx = self.get_index(row, col);
        self.cells[idx] = 1 & !self.cells[idx];
    }

    pub fn clear(&mut self) {


        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                self.cells[idx] = DEAD;
            }
        }
    }

    fn init_cells(width:u32, height:u32, fg_red:u8, fg_green:u8, fg_blue:u8) -> Vec<u8>{

        let cells: Vec<u8> = (0..width * height + 3)
            .map(|i| {
                if i < width * height {
                    if i % 2 == 0 || i % 7 == 0 {
                        ALIVE
                    } else {
                        DEAD
                    }
                } else if i == width * height {
                    fg_red
                } else if i == width * height + 1 {
                    fg_green
                } else {
                    fg_blue
                }
            })
            .collect();
        cells
    }

    pub fn initial_cells(& mut self) {

        self.cells = Universe::init_cells(self.width, self.height, self.fg_red, self.fg_green, self.fg_blue);
    }

    pub fn random(&mut self) {


        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                if js_sys::Math::random() < 0.5 {
                    self.cells[idx] = ALIVE;
                } else {
                self.cells[idx] = DEAD;
                }
            }
        }
    }


}



impl Default for Universe {
    fn default() -> Self {
        Universe::new()
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    /// Excluding the last three bytes which encode the alive
    pub fn get_cells(&self) -> &[u8]{
        &self.cells[..self.cells.len()-3]
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells:&[(u32,u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = ALIVE;
        }
    }
}