use crate::element::Element;
use crate::vhdl::direction::Direction;
use crate::vhdl::generic::Generic;
use crate::vhdl::port::Port;

#[derive(Clone)]
pub struct Binding {
    generic : bool,
    inner : String,
    direction : Direction,
    data_type : String,
    outer : String,
}

impl Binding {
    pub fn from_generic( generic : & Generic ) -> Binding{
        Binding { generic : true, inner : generic.get_name().to_string(),
                direction : Direction::IN, data_type : generic.get_data_type().to_string(),
                outer : String::new() }
    }

    pub fn from_port( port : & Port ) -> Binding {
        Binding { generic : false, inner : port.get_name().to_string(),
                direction : port.get_direction(), data_type : port.get_data_type().to_string(),
                outer : String::new() }
    }

    pub fn connect_to_generic( & mut self, generic : & Generic ) {
        self.outer = generic.get_name().to_string();
    }

    pub fn connect_to_port( & mut self, port : & Port ) {
        let _port_direction = & port.get_direction();
        if ! matches!( self.direction, _port_direction ) {
            panic!( "error: port direction mismatch!" );
        }
        if & self.data_type != port.get_data_type() {
            panic!( "error: port data type mismatch!" );
        }
        self.outer = port.get_name().to_string();
    }

    pub fn connect_by_name( & mut self, outer : & str ) {
        self.outer = outer.to_string();
    }

    pub fn get_inner( & self ) -> & String {
        & self.inner
    }

    pub fn get_direction( & self ) -> & Direction {
        & self.direction
    }

    pub fn get_data_type( & self ) -> & String {
        & self.data_type
    }

    pub fn get_outer( & self ) -> & String {
        & self.outer
    }
}

impl Element for Binding {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        if self.generic && self.outer.is_empty() {
            return source;
        }
        let indent_str = crate::util::indent( indent );
        source.push_str( & format!( "{}{} => {}", indent_str, self.inner, self.outer ) );
        return source;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_generic() {
        let binding = Binding::from_generic( & Generic::new( "test", "boolean" ) );
        assert_eq!( & binding.to_source_code( 0 ), "" );
    }

    #[test]
    fn from_port() {
        let binding = Binding::from_port( & Port::new( "test", Direction::OUT, "boolean" ) );
        assert_eq!( & binding.to_source_code( 0 ), "test => " );
    }

    #[test]
    fn connect_to_generic() {
        let generic = Generic::new( "extern", "boolean" );
        let mut binding = Binding::from_generic( & Generic::new( "test", "boolean" ) );
        binding.connect_to_generic( & generic );
        assert_eq!( & binding.to_source_code( 0 ), "test => extern" );
    }

    #[test]
    fn connect_to_port() {
        let port = Port::new( "extern", Direction::IN, "boolean" );
        let mut binding = Binding::from_port( & Port::new( "test", Direction::IN, "boolean" ) );
        binding.connect_to_port( & port );
        assert_eq!( & binding.to_source_code( 0 ), "test => extern" );
    }
}



