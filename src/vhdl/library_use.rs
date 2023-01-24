use crate::element::Element;
use crate::vhdl::keywords::*;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct LibraryUse {
    library : String,
    package : String,
    element : String,
}

impl LibraryUse {
    pub fn new( library : & str, package : & str ) -> LibraryUse {
        LibraryUse { library : String::from( library ), package : String::from( package ),
                element : ALL.to_string() }
    }
    pub fn new_with_element( library : & str, package : & str, element : & str ) -> LibraryUse {
        LibraryUse { library : String::from( library ), package : String::from( package ),
                element : String::from( element ) }
    }

    pub fn get_library_name( & self ) -> & String {
        & self.library
    }
}

impl Element for LibraryUse {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {}.{}.{};\n", indent_str, USE, self.library,
                self.package, self.element ) );

        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_use_all() {
        let c1 = LibraryUse::new( "ieee", "std_logic_1164" );

        assert_eq!(
            c1.to_source_code( 1 ),
            String::from( "    use ieee.std_logic_1164.all;\n" )
        );
    }

    #[test]
    fn library_use_element() {
        let c1 = LibraryUse::new_with_element( "ieee", "std_logic_1164", "std_logic" );

        assert_eq!(
            c1.to_source_code( 1 ),
            String::from( "    use ieee.std_logic_1164.std_logic;\n" )
        );
    }
}

