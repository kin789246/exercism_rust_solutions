#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        //unimplemented!( "Construct a ChessPosition struct, given the following rank, file: ({rank}, {file}). If the position is invalid return None." );
        if 0<=rank && rank<8 && 0<=file && file<8 {
            return Some(ChessPosition(rank, file))
        }
        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        //unimplemented!("Given the chess position {position:?}, construct a Queen struct.");
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        //unimplemented!("Determine if this Queen can attack the other Queen {other:?}");
        let is_diagonal: bool = (self.position.0 - other.position.0).abs() == (self.position.1 - other.position.1).abs();
        if self.position.0 == other.position.0 || self.position.1 == other.position.1 || is_diagonal {
            return true
        }
        false
    }
}
