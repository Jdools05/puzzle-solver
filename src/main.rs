mod piece;
use piece::Piece;

use std::io::Write;

fn main() {

    // time the search
    let start = std::time::Instant::now();

    // the tiles that are taken by the puzzle boarder
    let puzzle_boarder: u64 = 0x0303010101011FFF;

    // where we can reference the pieces
    let pieces_lookup: [Piece; 8] = create_pieces_lookup();

    // where we can reference the variants of the pieces
    let variants_lookup: [Vec<Piece>; 8] = pieces_lookup.iter().map(|piece| {piece.get_variants()}).collect::<Vec<Vec<Piece>>>().try_into().unwrap();

    // the solutions to the puzzle
    let mut all_solutions: Vec<[u64; 8]> = Vec::new();

    // used to count the number of attempted piece placements
    let mut counter = 0;

    // start the recursion
    solve_puzzle(puzzle_boarder, &variants_lookup, 0, &mut [0; 8], &mut all_solutions, &mut counter);

    println!("Number of iterations: {}", counter);

    println!("Number of solutions + invalids: {}", all_solutions.len());

    // end the timer and print
    let duration = start.elapsed();
    println!("Time elapsed is for search: {:?}", duration);

    // time the data writing
    let start = std::time::Instant::now();

    // create the bitmasks for the days and months
    let day_lookup: [u64; 31] = create_day_lookup();
    let month_lookup: [u64; 12] = create_month_lookup();

    // count the number of valid solutions
    let mut number_of_valid_solutions = 0;

    // used for saving the solutions
    let month_index_to_string: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    // loop over each valid date
    for (month, month_bitmask) in month_lookup.iter().enumerate() {
        for (day, day_bitmask) in day_lookup.iter().enumerate() {
            // make sure invalid dates are not checked like 31st of february
            if (![0, 2, 4, 6, 7, 9, 11].contains(&month) && day > 29) | (month == 1 && day > 28) {
                continue;
            }

            // open a file to write the solutions to
            // it should be in /solutions/month/day.txt
            let month_string = month_index_to_string[month];
            std::fs::create_dir_all(format!("solutions/{}", month_string)).unwrap();
            let mut file = std::fs::File::create(format!("solutions/{}/{}.txt", month_string, day + 1)).unwrap();
            // clear the file
            file.set_len(0).unwrap();

            // loop over each solution
            for solution in all_solutions.iter() {
                // reconstruct the solution
                let mut bitmap: u64 = 0;
                for piece in solution.iter() {
                    bitmap |= piece;
                }
                // check if the solution fits the date
                if (bitmap & month_bitmask) == 0 && (bitmap & day_bitmask) == 0 {
                    number_of_valid_solutions += 1;
                    // write the solution to the file
                    for piece in solution.iter() {
                        // write the piece in hex for readability
                        // base 64 would be more efficient but less readable
                        file.write_all(format!("{:016X} ", piece).as_bytes()).unwrap();
                    }
                    file.write_all(b"\n").unwrap();
                }
            }
        }
    }

    println!("Number of valid solutions: {}", number_of_valid_solutions);
    // end the timer and print
    let duration = start.elapsed();
    println!("Time elapsed is for data writing: {:?}", duration);

}

// create the lookup tables for the pieces by hand
fn create_pieces_lookup() -> [Piece; 8] {
    let mut pieces: [Piece; 8] = [Piece::new(0, 0, 0); 8];
    // P Piece
    pieces[0] = Piece::new(0x10303, 2, 3);
    
    // Corner Piece
    pieces[1] = Piece::new(0x10107, 3, 3);

    // U Piece
    pieces[2] = Piece::new(0x30103, 2, 3);

    // L Piece
    pieces[3] = Piece::new(0x1010103, 2, 4);

    // Z Piece
    pieces[4] = Piece::new(0x2030101, 2, 4);

    // Block Piece
    pieces[5] = Piece::new(0x30303, 2, 3);

    // S Piece
    pieces[6] = Piece::new(0x60203, 3, 3);

    // Y Piece
    pieces[7] = Piece::new(0x1030101, 2, 4);

    pieces
}

// create the bitmasks for the days
fn create_day_lookup() -> [u64; 31] {
    // allocate the array
    let mut day_lookup: [u64; 31] = [0; 31];
    for i in 0..31 {
        // fill the array with the bitmasks
        day_lookup[i] = 0b1 << (48 - (i+1) - (i / 7));
    }

    day_lookup
}

// create the bitmasks for the months
fn create_month_lookup () -> [u64; 12] {
    // allocate the array
    let mut month_lookup: [u64; 12] = [0; 12];
    for i in 0..12 {
        // fill the array with the bitmasks
        month_lookup[i] = 0b1 << (63 - i - (i / 6 * 2));
    }
    month_lookup
}

// solve the puzzle recursively this is where the magic happens
fn solve_puzzle(board: u64, pieces: &[Vec<Piece>; 8], index: u64, current_solution: &mut [u64; 8], all_solutions: &mut Vec<[u64; 8]>, counter: &mut u64) {

    // stopping condition
    // if we have placed all the pieces then we have a solution
    if index == 8 {
        // add the solution to the list of solutions
        all_solutions.push(current_solution.clone());
        return;
    }
    
        // loop over each variant
    for variant in pieces[index as usize].iter() {
        // loop over each position on the board minus the width and height of the piece
        for y in 0..(8-variant.height) {
            for x in 0..(8-variant.width) {
                // shift the piece to a new position
                let moved_piece: u64 = variant.shape << ((x+1) + (y+1) * 8);
                // increment the counter because we are trying to place a piece
                *counter += 1;

                // we can't place a piece on top of another piece
                // and this checks if it can be placed by comparing the bitmasks
                // if there is a "1" in the same spot in both bitmasks then they overlap
                // and we can't place the piece
                if (board & moved_piece) != 0 {
                    continue;
                }

                // place the piece on the board
                let new_board: u64 = board | moved_piece;

                // set the index of the current solution to the moved piece
                // this is used to keep track of the solution
                current_solution[index as usize] = moved_piece;

                // recurse but with the new board and the next piece in the list
                solve_puzzle(new_board, &pieces, index + 1, current_solution, all_solutions, counter);
            }
        }
    }
}


// tests to make sure the functions work
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_lookup() {
        let month_lookup = create_month_lookup();
        assert_eq!(month_lookup[0], 0x8000000000000000);
        assert_eq!(month_lookup[1], 0x4000000000000000);
        assert_eq!(month_lookup[2], 0x2000000000000000);
        assert_eq!(month_lookup[3], 0x1000000000000000);
        assert_eq!(month_lookup[4], 0x0800000000000000);
        assert_eq!(month_lookup[5], 0x0400000000000000);
        assert_eq!(month_lookup[6], 0x0080000000000000);
        assert_eq!(month_lookup[7], 0x0040000000000000);
        assert_eq!(month_lookup[8], 0x0020000000000000);
        assert_eq!(month_lookup[9], 0x0010000000000000);
        assert_eq!(month_lookup[10], 0x0008000000000000);
        assert_eq!(month_lookup[11], 0x0004000000000000);
    }

    #[test]
    fn test_day_lookup() {
        let day_lookup = create_day_lookup();
        assert_eq!(day_lookup[0], 0x0000800000000000);
        assert_eq!(day_lookup[1], 0x0000400000000000);
        assert_eq!(day_lookup[2], 0x0000200000000000);
        assert_eq!(day_lookup[3], 0x0000100000000000);
        assert_eq!(day_lookup[4], 0x0000080000000000);
        assert_eq!(day_lookup[5], 0x0000040000000000);
        assert_eq!(day_lookup[6], 0x0000020000000000);
        assert_eq!(day_lookup[7], 0x0000008000000000);
        assert_eq!(day_lookup[8], 0x0000004000000000);
        assert_eq!(day_lookup[9], 0x0000002000000000);
        assert_eq!(day_lookup[10], 0x0000001000000000);
        assert_eq!(day_lookup[11], 0x0000000800000000);
        assert_eq!(day_lookup[12], 0x0000000400000000);
        assert_eq!(day_lookup[13], 0x0000000200000000);
        assert_eq!(day_lookup[14], 0x0000000080000000);
        assert_eq!(day_lookup[15], 0x0000000040000000);
        assert_eq!(day_lookup[16], 0x0000000020000000);
        assert_eq!(day_lookup[17], 0x0000000010000000);
        assert_eq!(day_lookup[18], 0x0000000008000000);
        assert_eq!(day_lookup[19], 0x0000000004000000);
        assert_eq!(day_lookup[20], 0x0000000002000000);
        assert_eq!(day_lookup[21], 0x0000000000800000);
        assert_eq!(day_lookup[22], 0x0000000000400000);
        assert_eq!(day_lookup[23], 0x0000000000200000);
        assert_eq!(day_lookup[24], 0x0000000000100000);
        assert_eq!(day_lookup[25], 0x0000000000080000);
        assert_eq!(day_lookup[26], 0x0000000000040000);
        assert_eq!(day_lookup[27], 0x0000000000020000);
        assert_eq!(day_lookup[28], 0x0000000000008000);
        assert_eq!(day_lookup[29], 0x0000000000004000);
        assert_eq!(day_lookup[30], 0x0000000000002000);
    }

    #[test]
    fn test_bitmap_combination() {
        let bitmap: u64 = 0x0303010101011FFF;
        let piece: u64 = 0xE0C0000000000000;
        assert_eq!(bitmap | piece, 0xE3C3010101011FFF);
        assert_eq!(bitmap & piece, 0x0000000000000000);
    }
}
