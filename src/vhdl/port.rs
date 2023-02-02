use serde_derive::Deserialize;
use crate::vhdl::direction::Direction;
use crate::element::Element;

#[derive(Deserialize, Debug, Clone)]
pub struct Port {
    name : String,
    direction : Direction,
    data_type : String,
    #[serde(default)]
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

    pub fn clone_invert( & self ) -> Port {
        Port::new_with_default( & self.name, self.direction.get_inverted(), & self.data_type,
            & self.default )
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_direction( & self ) -> Direction {
        self.direction
    }

    pub fn get_data_type( & self ) -> & String {
        & self.data_type
    }

    pub fn set_name( & mut self, name : & str ) {
        self.name = name.to_string();
    }

    pub fn invert( & mut self ) {
        self.direction.invert();
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
    use std::error::Error;

    const INPUT : &'static str = "test : in boolean";
    const INPUT_WITH_DEFAULT : &'static str = "test : in boolean := true";
    const OUTPUT : &'static str = "test : out boolean";
    const INOUT : &'static str = "test : inout boolean";
    const BUFFER : &'static str = "test : buffer boolean";

    #[test]
    fn input_port() {
        let port = Port::new( "test", Direction::IN, "boolean" );
        assert_eq!( port.to_source_code( 0 ), INPUT.to_string() );
    }

    #[test]
    fn output_port() {
        let port = Port::new( "test", Direction::OUT, "boolean" );
        assert_eq!( port.to_source_code( 0 ), OUTPUT.to_string() );
    }

    #[test]
    fn inout_port() {
        let port = Port::new( "test", Direction::INOUT, "boolean" );
        assert_eq!( port.to_source_code( 0 ), INOUT.to_string() );
    }

    #[test]
    fn buffer_port() {
        let port = Port::new( "test", Direction::BUFFER, "boolean" );
        assert_eq!( port.to_source_code( 0 ), BUFFER.to_string() );
    }

    #[test]
    fn input_port_with_default() {
        let port = Port::new_with_default( "test", Direction::IN, "boolean", "true" );
        assert_eq!( port.to_source_code( 0 ), INPUT_WITH_DEFAULT.to_string() );
    }

    #[test]
    fn deserialize() -> Result< (), Box< dyn Error > > {
        let port : Port = serde_json::from_str(
            "{\"name\" : \"test\", \"direction\" : \"in\", \"data_type\" : \"boolean\"}" )?;
        assert_eq!( port.to_source_code( 0 ), INPUT.to_string() );
        Ok(())
    }

    #[test]
    fn deserialize_with_default() -> Result< (), Box< dyn Error > > {
        let port : Port = serde_json::from_str(
            "{\"name\" : \"test\", \"direction\" : \"in\", \"data_type\" : \"boolean\", \"default\" : \"true\"}" )?;
        assert_eq!( port.to_source_code( 0 ), INPUT_WITH_DEFAULT.to_string() );
        Ok(())
    }

    #[test]
    fn deserialize_invalid() {
        let ret : Result< Port, serde_json::Error > = serde_json::from_str( "\"invalid\"" );
        assert!( ret.is_err() );
    }
}

