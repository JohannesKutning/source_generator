use crate::element::Element;
use crate::vhdl::keywords::*;
use crate::vhdl::block_declarative_item::BlockDeclarativeItem;
use crate::vhdl::process_declarative_item::ProcessDeclarativeItem;

#[derive(Clone)]
pub struct ConstantDeclaration {
    name : String,
    data_type : String,
    default : String
}

impl ConstantDeclaration {
    pub fn new( name : & str, data_type : & str, default : & str )
            -> ConstantDeclaration {
        ConstantDeclaration { name : name.to_string(), data_type : data_type.to_string(),
                default : default.to_string() }
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_data_type( & self ) -> & String {
        & self.data_type
    }
}

impl Element for ConstantDeclaration {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {} : {}", indent_str, CONSTANT, self.name,
                self.data_type ));
        if ! self.default.is_empty() {
            source.push_str( & format!( " := {}", self.default ) );
        }
        source.push_str( ";\n" );

        return source;
    }
}

impl BlockDeclarativeItem for ConstantDeclaration {
}

impl ProcessDeclarativeItem for ConstantDeclaration {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create a constant declaration with default value.
     */
    #[test]
    fn constant_declaration_with_default() {
        let constant_declaration = ConstantDeclaration::new( "test", "boolean", "false" );

        assert_eq!(
            constant_declaration.to_source_code( 0 ),
            format!( "constant test : boolean := false;\n",  )
        );
    }
}

