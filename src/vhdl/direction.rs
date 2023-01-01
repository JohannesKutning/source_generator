#[derive(Debug, Clone, Copy)]
pub enum Direction {
    IN,
    OUT,
    INOUT,
    BUFFER
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
}

