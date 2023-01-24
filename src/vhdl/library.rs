use linked_hash_set::LinkedHashSet;

use crate::element::Element;
use crate::vhdl::library_use::LibraryUse;
use crate::vhdl::keywords::*;

#[derive(Clone)]
pub struct Library {
    name : String,
    uses : LinkedHashSet< LibraryUse >
}

impl Library {
    pub fn new( name : & str ) -> Library {
        Library { name : String::from( name ),
                uses : LinkedHashSet::new() }
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn add_use( & mut self, library_use : LibraryUse ) {
        self.uses.insert( library_use );
    }
}

impl Element for Library {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {};\n", indent_str, LIBRARY, self.name ) );
        for u in & self.uses {
            source.push_str( & u.to_source_code( indent + 1 ) );
        }

        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library() {
        let l = Library::new( "ieee" );

        assert_eq!(
            l.to_source_code( 0 ),
            String::from( "library ieee;\n" )
        );
    }

    #[test]
    fn library_with_use() {
        let mut l = Library::new( "ieee" );
        l.add_use( LibraryUse::new( "ieee", "std_logic_1164" ) );
        l.add_use( LibraryUse::new( "ieee", "std_logic_1164" ) );
        l.add_use( LibraryUse::new( "ieee", "numeric_std" ) );

        assert_eq!(
            l.to_source_code( 0 ),
            String::from( "library ieee;\n    use ieee.std_logic_1164.all;\n    use ieee.numeric_std.all;\n" )
        );
    }
}

