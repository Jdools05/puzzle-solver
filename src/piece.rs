use std::cmp::Ordering;



// this is the data structure that holds the piece information
// it derives these traits so that it can be sorted and compared
#[derive(Copy, Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct Piece {
    // the actual bitmask
    pub shape: u64,

    // the width and height of the piece
    pub width: u64,
    pub height: u64,
}

// returns a new piece with the same values
impl Clone for Piece {
    fn clone(&self) -> Piece {
        Piece {
            shape: self.shape,
            width: self.width,
            height: self.height,
        }
    }
}

// sorts the pieces by their shape, this is used to remove duplicates
impl Ord for Piece {
    fn cmp(&self, other: &Piece) -> Ordering {
        self.shape.cmp(&other.shape)
    }
}

impl Piece {
    // create a new piece with a bitmask and width and height
    pub fn new(shape: u64, width: u64, height: u64) -> Piece {
        Piece {
            shape,
            width,
            height,
        }
    }

    // rotate the piece 90 degrees to the right and return a new piece
    pub fn rotate_right(&self) -> Piece {
        // create a new bitmask
        let mut rotated_shape: u64 = 0;

        // loop over each bit in the bitmask
        for y in 0..self.height {
            for x in 0..self.width {
                // if the bit is a "1" then we need to rotate it
                // this form of rotation keeps the piece in the bottom right corner of the bitmask
                if self.shape & (0b1 << y * 8 + x) != 0 {
                    let (y, x) = (x, self.height - y -1);
                    rotated_shape |= 0b1 << y * 8 + x;
                }
            }
        }
        // return a new piece with the rotated bitmask
        Piece {
            shape: rotated_shape,
            width: self.height,
            height: self.width,
        }
    }

    // this mirrors the piece and returns a new piece
    pub fn mirror(&self) -> Piece {
        // create a new bitmask
        let mut mirrored_shape: u64 = 0;

        // loop over each bit in the bitmask
        for y in 0..self.height {
            for x in 0..self.width {

                // if the bit is a "1" then we need to mirror it
                if self.shape & (0b1 << y * 8 + x) != 0 {
                    let (y, x) = (y, self.width - x -1);
                    mirrored_shape |= 0b1 << y * 8 + x;
                }
            }
        }
        // return a new piece with the mirrored bitmask
        Piece {
            shape: mirrored_shape,
            width: self.width,
            height: self.height,
        }
    }

    // this returns a vector of all the variants of the piece
    pub fn get_variants(&self) -> Vec<Piece> {
        // allocate a vector of pieces
        let mut variants: Vec<Piece> = vec![Piece::new(0, 0, 0); 8];
        
        // create the first variant
        let mut rotated_piece: Piece = self.rotate_right();
        // loop over each variant 4 times because there are 4 rotations
        // and then mirror the piece and do it again
        for i in 0..2 {
            for j in 0..4 {
                // add the variant to the vector
                variants[i * 4 + j] = rotated_piece.clone();
                // rotate the piece
                rotated_piece = rotated_piece.rotate_right();
            }
            // mirror the piece
            rotated_piece = rotated_piece.mirror();
        }
        // sort the vector and remove duplicates
        variants.sort();
        variants.dedup();

        // print the bitmasks for debugging
        for i in 0..variants.len() {
            print!("0x{:X}, ", variants[i].shape);
        }
        println!();

        // return the vector
        variants
    }

    

}