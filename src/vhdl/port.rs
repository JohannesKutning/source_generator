use crate::vhdl::direction::Direction;
use crate::element::Element;

pub struct Port {
    name : String,
    direction : Direction,
    data_type : String,
    default : String
}

impl Port {
    pub fn new( name : & str, direction : Direction, data_type : & str )
            -> Port  {
        Port{ name : name.to_string(), direction : direction, data_type : data_type.to_string(),
                default : String::new() }

    }

    pub fn new_with_default( name : & str, direction : Direction, data_type : & str,
            default : & str ) -> Port  {
        Port{ name : name.to_string(), direction : direction, data_type : data_type.to_string(),
                default : default.to_string() }

    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn set_name( & mut self, name : & str ) {
        self.name = name.to_string();
    }

    pub fn invert( & self ) -> Port {
        Port::new_with_default( & self.name, self.direction.invert(), & self.data_type, & self.default )
    }
}

impl Element for Port {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        source.push_str( & format!( "{}{} : {} {}", indent_str, self.name, self.direction,
                self.data_type ) );

        if ! self.default.is_empty() {
            source.push_str( & format!( " := {}", self.default ) );
        }

        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_port() {
        let port = Port::new( "test", Direction::IN, "boolean" );

        assert_eq!(
            port.to_source_code( 0 ),
            String::from( "test : in boolean" )
        );
    }

    #[test]
    fn output_port() {
        let port = Port::new( "test", Direction::OUT, "boolean" );

        assert_eq!(
            port.to_source_code( 0 ),
            String::from( "test : out boolean" )
        );
    }

    #[test]
    fn inout_port() {
        let port = Port::new( "test", Direction::INOUT, "boolean" );

        assert_eq!(
            port.to_source_code( 0 ),
            String::from( "test : inout boolean" )
        );
    }

    #[test]
    fn buffer_port() {
        let port = Port::new( "test", Direction::BUFFER, "boolean" );

        assert_eq!(
            port.to_source_code( 0 ),
            String::from( "test : buffer boolean" )
        );
    }
}
