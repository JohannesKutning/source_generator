pub static NONE : u32 = 0;
pub static CLASS : u32 = 1;
pub static PARTIAL : u32 = 2;
pub static FULL : u32 = 3;

pub struct MatchIndex {
    pub index : usize,
    pub strength : u32,
}

impl MatchIndex {
    pub fn new() -> MatchIndex {
        MatchIndex { index : 0, strength : 0 }
    }

    pub fn update( & mut self, index : usize, strength : u32 ) {
        if self.strength < strength {
            self.index = index;
            self.strength = strength;
        }
    }

    pub fn strength( & self ) -> u32 {
        self.strength
    }

    pub fn position( & self ) -> usize {
        self.index
    }

    pub fn is_match( & self ) -> bool {
        self.strength > 0
    }
}
