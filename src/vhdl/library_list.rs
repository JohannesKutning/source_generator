use std::collections::HashMap;
use itertools::Itertools;
use crate::element::Element;
use crate::vhdl::library::Library;
use crate::vhdl::library_use::LibraryUse;

#[derive(Clone)]
pub struct LibraryList {
    libraries : HashMap< String, Library >
}

impl LibraryList {
    pub fn new() -> LibraryList {
        LibraryList { libraries : HashMap::new() }
    }

    pub fn contains( & self, library_name : & str ) -> bool {
        self.libraries.contains_key( library_name )
    }

    pub fn add_library( & mut self, library : Library ) {
        if ! self.libraries.contains_key( library.get_name() ) {
            self.libraries.insert( library.get_name().to_string(), library );
        }
    }

    pub fn add_library_use( & mut self, library_use : LibraryUse ) {
        let library_name = & library_use.get_library_name();
        self.add_library( Library::new( library_name ) );

        let library = self.get_library_mut( library_name );
        library.add_use( library_use );
    }

    fn get_library_mut( & mut self, library_name : & str ) -> & mut Library {
        self.libraries.get_mut( library_name ).unwrap()
    }
}

impl Element for LibraryList {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();

        if self.libraries.is_empty() {
            return source;
        }

        for name in self.libraries.keys().sorted() {
            source.push_str( & self.libraries[ name ].to_source_code( indent ) );
            source.push_str( "\n" )
        }
        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    const LIBRARIES : &'static str = concat!( "library ieee;\n",
        "    use ieee.std_logic_1164.all;\n",
        "    use ieee.numeric_std.all;\n",
        "\n",
        "library test;\n",
        "    use test.utility.all;\n",
        "\n" );

    /**
     * Create a library list with three entries
     */
    #[test]
    fn library_list() {
        let mut library_list = LibraryList::new();
        library_list.add_library_use( LibraryUse::new( "ieee", "std_logic_1164" ) );
        library_list.add_library_use( LibraryUse::new( "ieee", "numeric_std" ) );
        library_list.add_library_use( LibraryUse::new( "test", "utility" ) );

        assert_eq!(
            library_list.to_source_code( 0 ),
            format!( "{}", LIBRARIES )
        );
    }
}

