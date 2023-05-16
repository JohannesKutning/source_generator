use crate::element::Element;
use crate::element::to_source_code_list;
use crate::vhdl::design_unit::DesignUnit;
use crate::vhdl::keywords::*;
use crate::vhdl::single_line_comment::SingleLineComment;
use crate::vhdl::library_list::LibraryList;
use crate::vhdl::library::Library;
use crate::vhdl::library_use::LibraryUse;
use crate::vhdl::known_libraries::get_known_library_use;
use crate::vhdl::generic::Generic;
use crate::vhdl::port::Port;
use crate::vhdl::entity_interface::EntityInterface;
use crate::vhdl::vhdl_error::VhdlError;

#[derive(Clone)]
pub struct Entity {
    name : String,
    library : String,
    description : SingleLineComment,
    libraries : LibraryList,
    interfaces : Vec< EntityInterface >,
}

impl Entity {
    pub fn new( name : & str ) -> Entity {
        let mut entity = Entity { name : String::from( name ), library : "work".to_string(),
                description : SingleLineComment::new(), libraries : LibraryList::new(),
                interfaces : Vec::new() };
        entity.add_interface( & EntityInterface::new( "", "__default__" ) ).unwrap();
        return entity;
    }

    pub fn with_interface( name : & str, interface : & EntityInterface ) -> Entity {
        let mut entity = Entity::new( name );
        entity.add_interface( interface ).unwrap();
        return entity;
    }

    pub fn add_description( & mut self, text : & str ) {
        self.description = SingleLineComment::new_with_text( text );
    }

    pub fn contains_library( & self, library_name : & str ) -> bool {
        self.libraries.contains( library_name )
    }

    pub fn add_library( & mut self, library : Library ) {
        self.libraries.add_library( library );
    }

    pub fn add_library_use( & mut self, library_use : & LibraryUse ) {
        self.libraries.add_library_use( library_use.clone() );
    }

    pub fn add_missing_library_use( & mut self, data_type : & str ) {
        match get_known_library_use( data_type ) {
            Some( l ) => self.add_library_use( & l ),
            None => {},
        };
    }

    pub fn add_generic( & mut self, generic : Generic ) {
        self.add_missing_library_use( generic.get_data_type() );
        self.interfaces[ 0 ].add_generic( generic );
    }

    pub fn add_port( & mut self, port : Port ) {
        self.add_missing_library_use( port.get_data_type() );
        self.interfaces[ 0 ].add_port( port );
    }

    pub fn add_interface( & mut self, interface : & EntityInterface )
            -> Result< (), VhdlError > {
        if self.contains_interface( interface ) {
            return Err( VhdlError::new( & format!(
                "error: Entity {:?} already contains an interface of class {:?} and name {:?}",
                self.get_name(),
                interface.get_class(),
                interface.get_name() ) ) );
        }
        for data_type in interface.get_data_types() {
            self.add_missing_library_use( & data_type );
        }
        self.interfaces.push( interface.clone() );
        Ok(())
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_target_library( & self ) -> & String {
        & self.library
    }

    pub fn get_interfaces( & self ) -> & Vec< EntityInterface > {
        & self.interfaces
    }

    pub fn contains_interface( & self, interface : & EntityInterface ) -> bool {
        for i in & self.interfaces {
            if i.get_name() == interface.get_name() &&
                    i.get_class() == interface.get_class() {
                return true;
            }
        }
        return false;
    }

    pub fn get_generics( & self ) -> Vec< Generic > {
        let mut vec = Vec::new();
        for interface in & self.interfaces {
            for generic in interface.get_generics() {
                vec.push( generic.clone() );
            }
        }
        return vec;
    }

    fn generics_to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        let list_indent_str = crate::util::indent( indent + 1 );
        let generic_list = self.get_generics_boxed();
        if ! generic_list.is_empty() {
            let list = to_source_code_list( & generic_list,
                    & format!( ";\n{}", list_indent_str ) );
            source.push_str( & format!( "{}{} (\n{}{}\n{});\n",
                    indent_str, GENERIC, list_indent_str, list, indent_str ) );
        }
        return source;
    }

    fn get_generics_boxed( & self ) -> Vec< Box< dyn Element > > {
        let mut generic_list : Vec< Box< dyn Element > > = Vec::new();
        for interface in & self.interfaces {
            for generic in interface.get_generics() {
                generic_list.push( Box::new( generic.clone() ) );
            }
        }
        return generic_list;
    }

    fn ports_to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        let list_indent_str = crate::util::indent( indent + 1 );
        let port_list = self.get_ports();
        if ! port_list.is_empty() {
            let list = to_source_code_list( & port_list,
                    & format!( ";\n{}", list_indent_str ) );
            source.push_str( & format!( "{}{} (\n{}{}\n{});\n",
                    indent_str, PORT, list_indent_str, list, indent_str ) );
        }
        return source;
    }

    fn get_ports( & self ) -> Vec< Box< dyn Element > > {
        let mut port_list : Vec< Box< dyn Element > > = Vec::new();
        for interface in & self.interfaces {
            for port in interface.get_ports() {
                port_list.push( Box::new( port.clone() ) );
            }
        }
        return port_list;
    }
}

impl Element for Entity {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        source.push_str( & self.libraries.to_source_code( indent ) );

        if ! self.description.is_empty() {
            source.push_str( & self.description.to_source_code( indent ) );
        }
        source.push_str( & format!( "{}{} {} {}\n", indent_str, ENTITY, self.name, IS ) );
        source.push_str( & self.generics_to_source_code( indent + 1 ) );
        source.push_str( & self.ports_to_source_code( indent + 1 ) );
        source.push_str( & format!( "{}{}\n", indent_str, BEGIN ) );
        source.push_str( & format!( "{}{} {} {};\n", indent_str, END, ENTITY, self.name ) );

        return source;
    }
}

impl DesignUnit for Entity {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::path::Path;
    use crate::vhdl::direction::Direction;

    const NAME : &'static str = "test";
    const DESCRIPTION : &'static str = "-- A cool entity description\n";
    const HEADER : &'static str = "entity test is\n";
    const GENERICS : &'static str = concat!( "    generic (\n",
        "        GA : std_logic;\n",
        "        GB : integer := 5\n",
        "    );\n" );
    const PORTS : &'static str = concat!( "    port (\n",
        "        a : in std_logic;\n",
        "        b : out boolean;\n",
        "        c : inout std_logic_vector( 31 downto 0 );\n",
        "        d : buffer std_logic_vector( 31 downto 0 )\n",
        "    );\n" );
    const BEGIN : &'static str = "begin\n";
    const END : &'static str = "end entity test;\n";
    const USE_STD_LOGIC_1164 : &'static str = concat!( "library ieee;\n",
        "    use ieee.std_logic_1164.all;\n\n" );
    const LIBRARIES : &'static str = concat!( "library ieee;\n",
        "    use ieee.std_logic_1164.all;\n",
        "    use ieee.numeric_std.all;\n",
        "\n",
        "library test;\n",
        "    use test.utility.all;\n",
        "\n" );
    const INTERFACE : &'static str = concat!( "    generic (\n", "        A : integer := 0;\n",
        "        B : std_logic := '0';\n", "        C : boolean;\n", "        D : positive\n",
        "    );\n", "    port (\n", "        a : in integer := 0;\n",
        "        b : out std_logic := '0';\n", "        c : inout boolean;\n",
        "        d : buffer positive\n",
        "    );\n" );

    #[test]
    fn entity_frame() {
        let entity = Entity::new( NAME );
        assert_eq!( entity.to_source_code( 0 ), format!( "{}{}{}", HEADER, BEGIN, END ));
    }

    #[test]
    fn entity_with_description() {
        let mut entity = Entity::new( NAME );
        entity.add_description( "A cool entity description" );

        assert_eq!(
            entity.to_source_code( 0 ),
            format!( "{}{}{}{}", DESCRIPTION, HEADER, BEGIN, END )
        );
    }

    #[test]
    fn entity_with_ports() {
        let mut entity = Entity::new( NAME );
        entity.add_description( "A cool entity description" );
        entity.add_port( Port::new( "a", Direction::IN, "std_logic" ) );
        entity.add_port( Port::new( "b", Direction::OUT, "boolean" ) );
        entity.add_port( Port::new( "c", Direction::INOUT, "std_logic_vector( 31 downto 0 )" ) );
        entity.add_port( Port::new( "d", Direction::BUFFER, "std_logic_vector( 31 downto 0 )" ) );

        assert_eq!(
            entity.to_source_code( 0 ),
            format!( "{}{}{}{}{}{}", USE_STD_LOGIC_1164, DESCRIPTION, HEADER, PORTS, BEGIN, END )
        );
    }

    #[test]
    fn entity_with_generics_and_ports() {
        let mut entity = Entity::new( NAME );
        entity.add_description( "A cool entity description" );
        entity.add_generic( Generic::new( "GA", "std_logic" ) );
        entity.add_generic( Generic::new_with_default( "GB", "integer", "5" ) );
        entity.add_port( Port::new( "a", Direction::IN, "std_logic" ) );
        entity.add_port( Port::new( "b", Direction::OUT, "boolean" ) );
        entity.add_port( Port::new( "c", Direction::INOUT, "std_logic_vector( 31 downto 0 )" ) );
        entity.add_port( Port::new( "d", Direction::BUFFER, "std_logic_vector( 31 downto 0 )" ) );
        assert_eq!( entity.to_source_code( 0 ),
                format!( "{}{}{}{}{}{}{}", USE_STD_LOGIC_1164, DESCRIPTION, HEADER, GENERICS, PORTS, BEGIN, END ) );
    }

    #[test]
    fn entity_with_interface() -> Result< (), Box< dyn Error > > {
        let interface = EntityInterface::from_file_unnamed(
                Path::new( "tests/vhdl/interface.json" ) )?;

        let mut entity = Entity::new( NAME );
        entity.add_interface( & interface ).unwrap();
        assert_eq!( entity.to_source_code( 0 ),
            format!( "{}{}{}{}{}", USE_STD_LOGIC_1164, HEADER, INTERFACE, BEGIN, END ) );
        Ok(())
    }

    #[test]
    fn interface_already_exists_error() -> Result< (), Box< dyn Error > > {
        let interface = EntityInterface::from_file( "test",
                Path::new( "tests/vhdl/interface.json" ) )?;

        let mut entity = Entity::new( NAME );
        entity.add_interface( & interface ).unwrap();
        let ret = entity.add_interface( & interface );
        assert!( ret.is_err() );
        Ok(())
    }

    /**
     * Create a entity with libraries
     */
    #[test]
    fn entity_with_library() {
        let mut entity = Entity::new( NAME );
        entity.add_library( Library::new( "ieee" ) );
        entity.add_library_use( & LibraryUse::new( "ieee", "std_logic_1164" ) );
        entity.add_library_use( & LibraryUse::new( "ieee", "numeric_std" ) );
        entity.add_library_use( & LibraryUse::new( "test", "utility" ) );

        assert_eq!(
            entity.to_source_code( 0 ),
            format!( "{}{}{}{}", LIBRARIES, HEADER, BEGIN, END )
        );
    }
}

