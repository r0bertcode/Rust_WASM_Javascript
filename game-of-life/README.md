# Conway's Game of Life

This repo is source code from a tutorial in the docs, we implemenet Conway's game of life, ( https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life ) with the core game logic written in Rust, and then we compile it into WASM, and let JS read the cells by directly importing the memory from wasm ( yes, you can do that )

Example.

```
import { Universe, Cell } from "../pkg";
import { memory } from "../pkg/game_of_life_bg";

//...

  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

//...
```

-----


 If you want to learn more, I highly reccomend looking through the source code, the core login revolves around using a vector as Matrix, and traversing it and counting the values of surounding cells to calculate what the cell with be in the next generation ( tick ) of the game.

 Example. ( Tick function )

 ```
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }
 ```