use crate::element::Element;

pub struct Generic {
    name : String,
    data_type : String,
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

    /**
     * Create a generic without default value
     */
    #[test]
    fn generic() {
        let port = Generic::new( "test", "boolean" );

        assert_eq!(
            port.to_source_code( 0 ),
            String::from( "test : boolean" )
        );
    }

    /**
     * Create a generic with default value
     */
    #[test]
    fn generic_with_default() {
        let port = Generic::new_with_default( "test", "boolean", "true" );

        assert_eq!(
            port.to_source_code( 0 ),
            String::from( "test : boolean := true" )
        );
    }
}
