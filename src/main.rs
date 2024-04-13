use std::collections::HashSet;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

// Define the type for the key (tuple of two integers)
type Cell = (i32, i32);

// Might be useful to keep track of max coordinates for down the road when we want to render.
struct GameBoard{
    max_x: i32,
    max_y: i32,
    board: HashSet<Cell>
}

fn get_next_generation(prev_board: &GameBoard) -> GameBoard {
    // Instead of recalculating out board edges everytime we render just keep track here
    let mut new_max_x: i32 = prev_board.max_x; 
    let mut new_max_y: i32 = prev_board.max_y;
    
    // HashSet of cells that are alive in the next Generation
    let mut new_live_cells: HashSet<Cell> = HashSet::new();

    // Hashmap used to keep track of currently dead cells that could potentially come back.
    // mapping the coordinates of the dead cell to a counter keeping track of its alive neighbors.
    let mut lazarus_cells: HashMap<Cell, i32> = HashMap::new();

    for curr_cell in prev_board.board.iter() {
        let mut live_neighbors = 0;

        // For each of this cells neighbors
        for x in curr_cell.0-1..curr_cell.0+2 {
            for y in curr_cell.1-1..curr_cell.1+2 {
                // We dont want to count the current cell as a neighbor
                if (x, y) == *curr_cell {continue;}

                let neighbor_coordinates = (x as i32, y as i32);
                // if the current board had an alive cell at these coordinates, increment the live_neighbor count for this cell
                if prev_board.board.contains(&neighbor_coordinates) {
                    live_neighbors += 1;
                } 
                // else the neighbor is a dead cell, if not already, add it to the lazarus hashmap and increment by 1
                else { 
                    lazarus_cells.entry(neighbor_coordinates).and_modify(|count| *count += 1).or_insert(1);
                }    
            }
        }
        // If exacttly 2 or 3 live neighbors, cell survives 
        if live_neighbors == 2 || live_neighbors == 3 {
            new_live_cells.insert(*curr_cell);
        }
    }
    // Last step, any dead cell of interest that had 3 alive neighbors, comes to life.
    // Also if the new cell is bigger than our old max coords, increase those
    for (curr_dead_cell, alive_neighbors) in lazarus_cells.iter() {
        if *alive_neighbors == 3 {
            new_live_cells.insert(*curr_dead_cell);
            if curr_dead_cell.0 > new_max_x { new_max_x = curr_dead_cell.0;}
            if curr_dead_cell.1 > new_max_y { new_max_y = curr_dead_cell.1;}
        }
    } 

    return GameBoard {
        max_x: new_max_x,
        max_y: new_max_y,
        board: new_live_cells
     };
}

fn print_board(board: &GameBoard) {
    // Print rows with cell symbols
    for y in 0..board.max_y {
      for x in 0..board.max_x {
        let cell = (x as i32, y as i32);
        if board.board.contains(&cell) {
          print!("* "); // Print "*" for live cells
        } else {
          print!(". "); // Print "." for dead cells
        }
      }
      println!();
    }
  }

fn main() {
    let mut inital_board_cells: HashSet<Cell> = HashSet::new();
    inital_board_cells.insert((2, 2));
    inital_board_cells.insert((3, 2));
    inital_board_cells.insert((4, 2));
    inital_board_cells.insert((1, 3));
    inital_board_cells.insert((2, 3));
    inital_board_cells.insert((3, 3));

    let mut board = GameBoard {
        max_x: 6,
        max_y: 6,
        board: inital_board_cells
    };

    for _ in 0..10 {
        // Print the current board
        print_board(&board);
        println!();
    
        // Introduce a 2-second delay
        sleep(Duration::from_secs(2));
    
        // Calculate the next generation (replace with your function call)
        let next_board = get_next_generation(&board);
    
        // Update the board for the next iteration
        board = next_board;
      }
}
