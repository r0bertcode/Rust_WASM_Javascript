use wasm_bindgen::prelude::*;

#[wasm_bindgen]
// repr(u8) Will allow each cell to represnted as a single byte
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}
// Universe structure, we measure the width / heigth
// to calculate matrix cells
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    // Returns width
    pub fn width(&self) -> u32 {
        self.width
    }

    // Returns height
    pub fn height(&self) -> u32 {
        self.height
    }

    // Return's a raw pointer to the Cells buffer
    pub fn cells (&self) -> *const Cell {
        self.cells.as_ptr()
    }

    // Formula to get index of matrix from 1D Vector ( Returnings a U-size to point )
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    // Returns a count of live neighbors on the matrix
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // Traverse nearby cells
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                // If unactive, continue
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                // If not calculate values of neighbors
                let neighbor_row = (row + delta_row) % self.height;
                let neightbor_col = (column + delta_col) % self.width;
                // Get index with the function we defined above
                let idx = self.get_index(neighbor_row, neightbor_col);
                // Increase the count by value
                count += self.cells[idx] as u8;
            }
        }
        // Return total ' neighbor ' count
        count
    }

    // Inifinite tick function, is responsible for updating the Cell's vector every generation
    pub fn tick(&mut self) {
        // Clone a mutable version of the current universe vector
        let mut next = self.cells.clone();

        // traverse matrix
        for row in 0..self.height {
            for col in 0..self.width {
                // get current index
                let idx = self.get_index(row, col);
                // get current cell
                let cell = self.cells[idx];
                // count neighbors
                let live_neighbors = self.live_neighbor_count(row, col);

                // based on live_neighors count, we determine the cells fate
                let next_cell = match (cell, live_neighbors) {
                    // 1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // 2. Any live cell with two or three live neighbours lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                // we mutate the copy of the universe
                next[idx] = next_cell;
            }
        }

        // we set the copy to be the new universe
        self.cells = next;
    }

    #[wasm_bindgen(constructor)]
    pub fn new() -> Universe {
        // Define height , width of our universe ( Cells x Cells )
        let width = 466;
        let height = 306;

        let cells = (0..width * height)
            .map(|i| {
                // Depending on cell number, spawn an alive or dead cell
                if i % 2 == 0 || i % 5 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        // Return the new universe
        Universe {
            width,
            height,
            cells,
        }
    }

    // Render if we want javascript to render through a string using a <pre> tag
    // pub fn render(&self) -> String {
    //     self.to_string()
    // }
}
