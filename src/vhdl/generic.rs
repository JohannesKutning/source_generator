use serde_derive::Deserialize;
use crate::element::Element;

#[derive(Deserialize, Debug, Clone)]
pub struct Generic {
    name : String,
    data_type : String,
    #[serde(default)]
    default : String
}

impl Generic {
    pub fn new( name : & str, data_type : & str )
            -> Generic  {
        Generic{ name : name.to_string(), data_type : data_type.to_string(),
                default : String::new() }

    }

    pub fn new_with_default( name : & str, data_type : & str,
            default : & str ) -> Generic  {
        Generic{ name : name.to_string(), data_type : data_type.to_string(),
                default : default.to_string() }

    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn set_name( & mut self, name : & str ) {
        self.name = name.to_string();
    }

    pub fn get_data_type( & self ) -> & String {
        & self.data_type
    }

    pub fn has_default( & self ) -> bool {
        ! self.default.is_empty()
    }
}

impl Element for Generic {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        source.push_str( & format!( "{}{} : {}", indent_str, self.name,
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

    const GENERIC : &'static str = "test : boolean";
    const GENERIC_WITH_DEFAULT : &'static str = "test : boolean := true";

    #[test]
    fn generic() {
        let generic = Generic::new( "test", "boolean" );
        assert_eq!( generic.to_source_code( 0 ), GENERIC.to_string() );
    }

    #[test]
    fn generic_with_default() {
        let generic = Generic::new_with_default( "test", "boolean", "true" );
        assert_eq!( generic.to_source_code( 0 ), GENERIC_WITH_DEFAULT.to_string() );
    }

    #[test]
    fn deserialize() -> Result< (), Box< dyn Error > > {
        let generic : Generic = serde_json::from_str(
            "{\"name\" : \"test\", \"data_type\" : \"boolean\"}" )?;
        assert_eq!( generic.to_source_code( 0 ), GENERIC.to_string() );
        Ok(())
    }

    #[test]
    fn deserialize_with_default() -> Result< (), Box< dyn Error > > {
        let generic : Generic = serde_json::from_str(
            "{\"name\" : \"test\", \"data_type\" : \"boolean\", \"default\" : \"true\"}" )?;
        assert_eq!( generic.to_source_code( 0 ), GENERIC_WITH_DEFAULT.to_string() );
        Ok(())
    }

    #[test]
    fn deserialize_invalid() {
        let ret : Result< Generic, serde_json::Error > = serde_json::from_str( "\"invalid\"" );
        assert!( ret.is_err() );
    }
}

