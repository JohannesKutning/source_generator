use std::error::Error;
use std::fs;
use std::path::Path;
use serde_derive::Deserialize;
use serde_json_schema::Schema;
use crate::vhdl::generic::Generic;
use crate::vhdl::port::Port;

#[derive(Deserialize, Debug, Clone)]
pub struct EntityInterface {
    #[serde(default)]
    name : String,
    class : String,
    #[serde(default)]
    generics : Vec< Generic >,
    #[serde(default)]
    ports : Vec< Port >,
}

impl EntityInterface {
    pub fn new( name : & str, class : & str ) -> EntityInterface {
        EntityInterface { name : name.to_string(), class : class.to_string(),
            generics : Vec::new(), ports : Vec::new() }
    }

    pub fn new_unnamed( class : & str ) -> EntityInterface {
        EntityInterface::new( "", class )
    }

    pub fn from_file( name : & str, filename : & Path ) -> Result< EntityInterface, Box< dyn Error > > {
        let mut interface = EntityInterface::from_file_unnamed( filename )?;
        interface.rename( name );
        Ok( interface )
    }

    pub fn from_file_unnamed( file : & Path ) -> Result< EntityInterface, Box< dyn Error > > {
        let schema = EntityInterface::read_schema()?;
        let interface = EntityInterface::read_and_validate_description( file, & schema )?;
        Ok( interface )
    }

    pub fn clone_invert( & self ) -> EntityInterface {
        let mut inverted = EntityInterface::new( & self.name, & self.class );
        for generic in self.get_generics() {
            inverted.add_generic( generic.clone() );
        }
        for port in self.get_ports() {
            inverted.add_port( port.clone_invert() );
        }
        return inverted;
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

    pub fn rename( & mut self, name : & str ) {
        self.name = name.to_string();
    }

    fn read_schema() -> Result< Schema, Box< dyn Error > > {
        let schema_file_name = "data/schema/entity_interface.json";
        let schema_str = fs::read_to_string( schema_file_name )?;
        let schema = Schema::try_from( schema_str )?;
        Ok( schema )
    }

    fn read_and_validate_description( file : & Path, schema : & Schema )
            -> Result< EntityInterface, Box< dyn Error > > {
        let module_str = fs::read_to_string( file )?;
        let module_json : serde_json::Value = serde_json::from_str( & module_str )?;
        match schema.validate( & module_json ) {
            Ok(_)   => {},
            Err( err ) => { eprintln!( "Failed to validate the {:?}\n    with error {:?}",
                file.to_string_lossy(), err ) }, };
        let description : EntityInterface = serde_json::from_str( & module_str )?;
        Ok( description )
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
        let mut interface = EntityInterface::new_unnamed( "test" );
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
        let mut interface = EntityInterface::new_unnamed( "test" );
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

    #[test]
    fn interface_from_json() -> Result< (), Box< dyn Error > > {
        let interface = EntityInterface::from_file_unnamed(
                Path::new( "tests/vhdl/interface.json" ) )?;
        let mut source = String::new();
        for generic in interface.get_generics() {
            source.push_str( & format!( "{}", generic.to_source_code( 0 ) ) );
        }
        for port in interface.get_ports() {
            source.push_str( & format!( "{}", port.to_source_code( 0 ) ) );
        }
        assert_eq!( format!( "{}{}", GENERICS, PORTS ), source );
        Ok(())
    }

    /**
     * Invert an entity interface with multiple generics and ports
     */
    #[test]
    fn invert_interface() {
        let mut interface = EntityInterface::new_unnamed( "test" );
        interface.add_generic( Generic::new_with_default( "A", "integer", "0" ) );
        interface.add_generic( Generic::new_with_default( "B", "std_logic", "'0'" ) );
        interface.add_generic( Generic::new( "C", "boolean" ) );
        interface.add_generic( Generic::new( "D", "positive" ) );
        interface.add_port( Port::new_with_default( "a", Direction::IN, "integer", "0" ) );
        interface.add_port( Port::new_with_default( "b", Direction::OUT, "std_logic", "'0'" ) );
        interface.add_port( Port::new( "c", Direction::INOUT, "boolean" ) );
        interface.add_port( Port::new( "d", Direction::BUFFER, "positive" ) );
        let inverted = interface.clone_invert();
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


