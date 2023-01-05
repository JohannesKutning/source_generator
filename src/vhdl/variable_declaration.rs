use crate::element::Element;
use crate::vhdl::keywords::*;
use crate::vhdl::process_declarative_item::ProcessDeclarativeItem;

pub struct VariableDeclaration {
    name : String,
    data_type : String,
    default : String
}

impl VariableDeclaration {
    pub fn new( name : & str, data_type : & str ) -> VariableDeclaration {
        VariableDeclaration { name : name.to_string(), data_type : data_type.to_string(),
                default : String::new() }
    }

    pub fn new_with_default( name : & str, data_type : & str, default : & str )
            -> VariableDeclaration {
        VariableDeclaration { name : name.to_string(), data_type : data_type.to_string(),
                default : default.to_string() }
    }
}

impl Element for VariableDeclaration {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {} : {}", indent_str, VARIABLE, self.name,
                self.data_type ));
        if ! self.default.is_empty() {
            source.push_str( & format!( " := {}", self.default ) );
        }
        source.push_str( ";\n" );

        return source;
    }
}

impl ProcessDeclarativeItem for VariableDeclaration {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create a variable declaration with no default value.
     */
    #[test]
    fn variable_declaration() {
        let variable_declaration = VariableDeclaration::new( "test", "boolean" );

        assert_eq!(
            variable_declaration.to_source_code( 0 ),
            format!( "variable test : boolean;\n",  )
        );
    }

    /**
     * Create a variable declaration with default value.
     */
    #[test]
    fn variable_declaration_with_default() {
        let variable_declaration = VariableDeclaration::new_with_default( "test", "boolean", "false" );

        assert_eq!(
            variable_declaration.to_source_code( 0 ),
            format!( "variable test : boolean := false;\n",  )
        );
    }
}

