use crate::element::Element;
use crate::vhdl::design_unit::DesignUnit;
use crate::vhdl::keywords::*;
use crate::vhdl::single_line_comment::SingleLineComment;
use crate::vhdl::library_list::LibraryList;
use crate::vhdl::library::Library;
use crate::vhdl::library_use::LibraryUse;
use crate::vhdl::generic_list::GenericList;
use crate::vhdl::generic::Generic;
use crate::vhdl::port_list::PortList;
use crate::vhdl::port::Port;
use crate::vhdl::entity_interface::EntityInterface;

pub struct Entity {
    name : String,
    library : String,
    description : SingleLineComment,
    libraries : LibraryList,
    generics : GenericList,
    ports : PortList,
    interfaces : Vec< EntityInterface >,
}

impl Entity {
    pub fn new( name : & str ) -> Entity {
        Entity { name : String::from( name ), library : "work".to_string(),
                description : SingleLineComment::new(), libraries : LibraryList::new(),
                generics : GenericList::new(), ports : PortList::new(),
                interfaces : Vec::new() }
    }

    pub fn with_interface( name : & str, interface : EntityInterface ) -> Entity {
        let mut entity = Entity::new( name );
        entity.add_interface( interface );
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

    pub fn add_library_use( & mut self, library_use : LibraryUse ) {
        self.libraries.add_library_use( library_use );
    }

    pub fn add_generic( & mut self, generic : Generic ) {
        self.generics.add_generic( generic );
    }

    pub fn add_port( & mut self, port : Port ) {
        self.ports.add_port( port );
    }

    pub fn add_interface( & mut self, interface : EntityInterface ) {
        for generic in interface.get_generics() {
            self.add_generic( generic.clone() );
        }
        for port in interface.get_ports() {
            self.add_port( port.clone() );
        }
        self.interfaces.push( interface );
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_target_library( & self ) -> & String {
        & self.library
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
        source.push_str( & self.generics.to_source_code( indent ) );
        source.push_str( & self.ports.to_source_code( indent ) );
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

    /**
     * Create a entity with no description and content.
     */
    #[test]
    fn entity_frame() {
        let entity = Entity::new( NAME );
        assert_eq!( entity.to_source_code( 0 ), format!( "{}{}{}", HEADER, BEGIN, END ));
    }

    /**
     * Create a entity with description but no content.
     */
    #[test]
    fn entity_with_description() {
        let mut entity = Entity::new( NAME );
        entity.add_description( "A cool entity description" );

        assert_eq!(
            entity.to_source_code( 0 ),
            format!( "{}{}{}{}", DESCRIPTION, HEADER, BEGIN, END )
        );
    }

    /**
     * Create a entity with description and a port list.
     */
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
            format!( "{}{}{}{}{}", DESCRIPTION, HEADER, PORTS, BEGIN, END )
        );
    }

    /**
     * Create a entity with description and a port list.
     */
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
                format!( "{}{}{}{}{}{}", DESCRIPTION, HEADER, GENERICS, PORTS, BEGIN, END ) );
    }

    /**
     * Create an entity with an interface
     */
    #[test]
    fn entity_with_interface() -> Result< (), Box< dyn Error > > {
        let interface = EntityInterface::from_file_unnamed( Path::new( "tests/interface.json" ) )?;
        let mut entity = Entity::new( NAME );
        entity.add_interface( interface );
        assert_eq!( entity.to_source_code( 0 ),
            format!( "{}{}{}{}", HEADER, INTERFACE, BEGIN, END ) );
        Ok(())
    }

    /**
     * Create a entity with libraries
     */
    #[test]
    fn entity_with_library() {
        let mut entity = Entity::new( NAME );
        entity.add_library( Library::new( "ieee" ) );
        entity.add_library_use( LibraryUse::new( "ieee", "std_logic_1164" ) );
        entity.add_library_use( LibraryUse::new( "ieee", "numeric_std" ) );
        entity.add_library_use( LibraryUse::new( "test", "utility" ) );

        assert_eq!(
            entity.to_source_code( 0 ),
            format!( "{}{}{}{}", LIBRARIES, HEADER, BEGIN, END )
        );
    }
}

