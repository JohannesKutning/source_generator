use serde_derive::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    IN,
    OUT,
    INOUT,
    BUFFER
}

impl Direction {
    pub fn invert( & mut self ) {
        match self {
            Direction::IN => { * self = Direction::OUT },
            Direction::OUT => { * self = Direction::IN },
            Direction::INOUT => { * self = Direction::INOUT },
            Direction::BUFFER => { * self = Direction::IN },
        }
    }

    pub fn get_inverted( & self ) -> Direction {
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
    use std::error::Error;
    use std::matches;

    #[test]
    fn deserialize_in() -> Result< (), Box< dyn Error > > {
        let direction : Direction = serde_json::from_str( "\"in\"" )?;
        assert!( matches!( direction, Direction::IN ) );
        Ok(())
    }

    #[test]
    fn deserialize_out() -> Result< (), Box< dyn Error > > {
        let direction : Direction = serde_json::from_str( "\"out\"" )?;
        assert!( matches!( direction, Direction::OUT ) );
        Ok(())
    }

    #[test]
    fn deserialize_inout() -> Result< (), Box< dyn Error > > {
        let direction : Direction = serde_json::from_str( "\"inout\"" )?;
        assert!( matches!( direction, Direction::INOUT ) );
        Ok(())
    }

    #[test]
    fn deserialize_buffer() -> Result< (), Box< dyn Error > > {
        let direction : Direction = serde_json::from_str( "\"buffer\"" )?;
        assert!( matches!( direction, Direction::BUFFER ) );
        Ok(())
    }

    #[test]
    fn deserialize_invalid() {
        let ret : Result< Direction, serde_json::Error > = serde_json::from_str( "\"invalid\"" );
        assert!( ret.is_err() );
    }

    #[test]
    fn in_to_string() {
        assert_eq!( Direction::IN.to_string(), String::from( "in" ) );
    }

    #[test]
    fn out_to_string() {
        assert_eq!( Direction::OUT.to_string(), String::from( "out" ) );
    }

    #[test]
    fn inout_to_string() {
        assert_eq!( Direction::INOUT.to_string(), String::from( "inout" ) );
    }

    #[test]
    fn buffer_to_string() {
        assert_eq!( Direction::BUFFER.to_string(), String::from( "buffer" ) );
    }

    #[test]
    fn invert_in() {
        let mut direction = Direction::IN;
        direction.invert();
        assert_eq!( direction.to_string(), String::from( "out" ) );
    }

    #[test]
    fn invert_out() {
        let mut direction = Direction::OUT;
        direction.invert();
        assert_eq!( direction.to_string(), String::from( "in" ) );
    }

    #[test]
    fn invert_inout() {
        let mut direction = Direction::INOUT;
        direction.invert();
        assert_eq!( direction.to_string(), String::from( "inout" ) );
    }

    #[test]
    fn invert_buffer() {
        let mut direction = Direction::BUFFER;
        direction.invert();
        assert_eq!( direction.to_string(), String::from( "in" ) );
    }

    #[test]
    fn get_inverted_in() {
        assert_eq!( Direction::IN.get_inverted().to_string(), String::from( "out" ) );
    }

    #[test]
    fn get_inverted_out() {
        assert_eq!( Direction::OUT.get_inverted().to_string(), String::from( "in" ) );
    }

    #[test]
    fn get_inverted_inout() {
        assert_eq!( Direction::INOUT.get_inverted().to_string(), String::from( "inout" ) );
    }

    #[test]
    fn get_inverted_buffer() {
        assert_eq!( Direction::BUFFER.get_inverted().to_string(), String::from( "in" ) );
    }
}

