use crate::element::Element;
use crate::vhdl::generic::Generic;
use crate::vhdl::keywords::GENERIC;

pub struct GenericList {
    generics : Vec< Generic >
}

impl GenericList {
    pub fn new() -> GenericList {
        GenericList { generics : Vec::new() }
    }

    pub fn add_generic( & mut self, generic : Generic ) {
        self.generics.push( generic );
    }
}

impl Element for GenericList {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();

        if self.generics.is_empty() {
            return source;
        }

        let indent_str = crate::util::indent( indent + 1 );
        source.push_str( & format!( "{}{} (\n", indent_str, GENERIC ) );

        for ( pos, port ) in self.generics.iter().enumerate() {
            source.push_str( & port.to_source_code( indent + 2 ) );
            if pos < self.generics.len() - 1 {
                source.push_str( ";" );
            }
            source.push_str( "\n" );
        }
        source.push_str( & format!( "{});\n", indent_str ) );

        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const GENERICS : &'static str = concat!( "    generic (\n", "        a : integer := 0;\n",
        "        b : std_logic := '0';\n", "        c : boolean\n", "    );\n" );

    /**
     * Create a library list with three entries
     */
    #[test]
    fn generic_list() {
        let mut generic_list = GenericList::new();
        generic_list.add_generic( Generic::new_with_default( "a", "integer", "0" ) );
        generic_list.add_generic( Generic::new_with_default( "b", "std_logic", "'0'" ) );
        generic_list.add_generic( Generic::new( "c", "boolean" ) );

        assert_eq!(
            generic_list.to_source_code( 0 ),
            format!( "{}", GENERICS )
        );
    }
}

