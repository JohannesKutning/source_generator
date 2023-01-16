#[derive(Debug, Clone, Copy)]
pub enum Direction {
    IN,
    OUT,
    INOUT,
    BUFFER
}

impl Direction {
    pub fn invert( & self ) -> Direction {
        match self {
            Direction::IN => { Direction::OUT },
            Direction::OUT => { Direction::IN },
            Direction::INOUT => { Direction::INOUT },
            Direction::BUFFER => { Direction::IN },
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt( & self, f : & mut std::fmt::Formatter ) -> std::fmt::Result {
        match self {
            Direction::IN => write!( f, "{}", crate::vhdl::keywords::IN ),
            Direction::OUT => write!( f, "{}", crate::vhdl::keywords::OUT ),
            Direction::INOUT => write!( f, "{}", crate::vhdl::keywords::INOUT ),
            Direction::BUFFER => write!( f, "{}", crate::vhdl::keywords::BUFFER ),
        }
    }
}


//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_in() {
        assert_eq!( Direction::IN.to_string(), String::from( "in" ) );
    }

    #[test]
    fn invert_in() {
        assert_eq!( Direction::IN.invert().to_string(), String::from( "out" ) );
    }

    #[test]
    fn invert_out() {
        assert_eq!( Direction::OUT.invert().to_string(), String::from( "in" ) );
    }

    #[test]
    fn invert_inout() {
        assert_eq!( Direction::INOUT.invert().to_string(), String::from( "inout" ) );
    }

    #[test]
    fn invert_buffer() {
        assert_eq!( Direction::BUFFER.invert().to_string(), String::from( "in" ) );
    }
}

