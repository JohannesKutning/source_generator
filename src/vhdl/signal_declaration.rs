use crate::element::Element;
use crate::vhdl::keywords::*;
use crate::vhdl::block_declarative_item::BlockDeclarativeItem;

#[derive(PartialEq, Clone)]
pub struct SignalDeclaraion {
    name : String,
    data_type : String,
    default : String
}

impl SignalDeclaraion {
    pub fn new( name : & str, data_type : & str ) -> SignalDeclaraion {
        SignalDeclaraion { name : name.to_string(), data_type : data_type.to_string(),
                default : String::new() }
    }

    pub fn new_with_default( name : & str, data_type : & str, default : & str )
            -> SignalDeclaraion {
        SignalDeclaraion { name : name.to_string(), data_type : data_type.to_string(),
                default : default.to_string() }
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_data_type( & self ) -> & String {
        & self.data_type
    }
}

impl Element for SignalDeclaraion {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {} : {}", indent_str, SIGNAL, self.name,
                self.data_type ));
        if ! self.default.is_empty() {
            source.push_str( & format!( " := {}", self.default ) );
        }
        source.push_str( ";\n" );

        return source;
    }
}

impl BlockDeclarativeItem for SignalDeclaraion {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create a signal declaration with no default value.
     */
    #[test]
    fn signal_declaration() {
        let signal_declaration = SignalDeclaraion::new( "test", "boolean" );

        assert_eq!(
            signal_declaration.to_source_code( 0 ),
            format!( "signal test : boolean;\n",  )
        );
    }

    /**
     * Create a signal declaration with default value.
     */
    #[test]
    fn signal_declaration_with_default() {
        let signal_declaration = SignalDeclaraion::new_with_default( "test", "boolean", "false" );

        assert_eq!(
            signal_declaration.to_source_code( 0 ),
            format!( "signal test : boolean := false;\n",  )
        );
    }
}

