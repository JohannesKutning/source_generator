use crate::vhdl::generic::Generic;
use crate::vhdl::port::Port;
use crate::vhdl::entity_interface_description::EntityInterfaceDescription;

pub struct EntityInterface {
    name : String,
    generics : Vec< Generic >,
    ports : Vec< Port >,
}

impl EntityInterface {
    pub fn new( name : & str ) -> EntityInterface {
        EntityInterface { name : name.to_string(), generics : Vec::new(),
            ports : Vec::new() }
    }

    pub fn from_json( _description : & EntityInterfaceDescription ) -> EntityInterface {
        EntityInterface { name : "".to_string(), generics : Vec::new(),
            ports : Vec::new() }
    }

    pub fn add_generic( & mut self, generic : Generic ) {
        self.generics.push( generic );
    }

    pub fn add_port( & mut self, port : Port ) {
        self.ports.push( port );
    }

    pub fn get_generics( & self ) -> & Vec< Generic > {
        & self.generics
    }

    pub fn get_ports( & self ) -> & Vec< Port > {
        & self.ports
    }

    pub fn invert( & self ) -> EntityInterface {
        let mut inverted = EntityInterface::new( & self.name );
        for generic in self.get_generics() {
            inverted.add_generic( generic.clone() );
        }
        for port in self.get_ports() {
            inverted.add_port( port.invert() );
        }
        return inverted;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::Element;
    use crate::vhdl::direction::Direction;

    const PORTS : &'static str = concat!( "a : in integer := 0",
        "b : out std_logic := '0'", "c : inout boolean",
        "d : buffer positive" );

    const GENERICS : &'static str = concat!( "A : integer := 0",
        "B : std_logic := '0'", "C : boolean",
        "D : positive" );


    const INVERTED : &'static str = concat!( "A : integer := 0",
        "B : std_logic := '0'", "C : boolean",
        "D : positive", "a : out integer := 0",
        "b : in std_logic := '0'", "c : inout boolean",
        "d : in positive" );

    /**
     * Create an entity interface with multiple ports but no generics
     */
    #[test]
    fn port_interface() {
        let mut interface = EntityInterface::new( "test" );
        interface.add_port( Port::new_with_default( "a", Direction::IN, "integer", "0" ) );
        interface.add_port( Port::new_with_default( "b", Direction::OUT, "std_logic", "'0'" ) );
        interface.add_port( Port::new( "c", Direction::INOUT, "boolean" ) );
        interface.add_port( Port::new( "d", Direction::BUFFER, "positive" ) );
        let mut source = String::new();
        for port in interface.get_ports() {
            source.push_str( & format!( "{}", port.to_source_code( 0 ) ) );
        }
        assert_eq!( PORTS, source );
    }

    /**
     * Create an entity interface with multiple generics but no ports
     */
    #[test]
    fn generic_interface() {
        let mut interface = EntityInterface::new( "test" );
        interface.add_generic( Generic::new_with_default( "A", "integer", "0" ) );
        interface.add_generic( Generic::new_with_default( "B", "std_logic", "'0'" ) );
        interface.add_generic( Generic::new( "C", "boolean" ) );
        interface.add_generic( Generic::new( "D", "positive" ) );
        let mut source = String::new();
        for generic in interface.get_generics() {
            source.push_str( & format!( "{}", generic.to_source_code( 0 ) ) );
        }
        assert_eq!( GENERICS, source );
    }

    /**
     * Create an entity interface with multiple generics but no ports
     */
    #[test]
    fn invert_interface() {
        let mut interface = EntityInterface::new( "test" );
        interface.add_generic( Generic::new_with_default( "A", "integer", "0" ) );
        interface.add_generic( Generic::new_with_default( "B", "std_logic", "'0'" ) );
        interface.add_generic( Generic::new( "C", "boolean" ) );
        interface.add_generic( Generic::new( "D", "positive" ) );
        interface.add_port( Port::new_with_default( "a", Direction::IN, "integer", "0" ) );
        interface.add_port( Port::new_with_default( "b", Direction::OUT, "std_logic", "'0'" ) );
        interface.add_port( Port::new( "c", Direction::INOUT, "boolean" ) );
        interface.add_port( Port::new( "d", Direction::BUFFER, "positive" ) );
        let inverted = interface.invert();
        let mut source = String::new();
        for generic in inverted.get_generics() {
            source.push_str( & format!( "{}", generic.to_source_code( 0 ) ) );
        }
        for port in inverted.get_ports() {
            source.push_str( & format!( "{}", port.to_source_code( 0 ) ) );
        }
        assert_eq!( INVERTED, source );
    }
}


