use crate::element::Element;
use crate::element::to_source_code_list;
use crate::vhdl::generic::Generic;
use crate::vhdl::keywords::GENERIC;

pub struct GenericList {
    generics : Vec< Box< dyn Element > >
}

impl GenericList {
    pub fn new() -> GenericList {
        GenericList { generics : Vec::new() }
    }

    pub fn add_generic( & mut self, generic : Generic ) {
        self.generics.push( Box::new( generic ) );
    }
}

impl Element for GenericList {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        if self.generics.is_empty() {
            return source;
        }

        let indent_str = crate::util::indent( indent + 1 );
        let list_indent_str = crate::util::indent( indent + 2 );
        let list = to_source_code_list( & self.generics, & format!( ";\n{}", list_indent_str ) );
        source.push_str( & format!( "{}{} (\n{}{}\n{});\n", indent_str, GENERIC, list_indent_str,
                list, indent_str ) );
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

