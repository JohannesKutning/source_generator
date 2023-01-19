use crate::element::Element;
use crate::element::to_source_code_list;
use crate::vhdl::port::Port;
use crate::vhdl::keywords::PORT;

pub struct PortList {
    ports : Vec< Box< dyn Element > >
}

impl PortList {
    pub fn new() -> PortList {
        PortList { ports : Vec::new() }
    }

    pub fn add_port( & mut self, port : Port ) {
        self.ports.push( Box::new( port ) );
    }
}

impl Element for PortList {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        if self.ports.is_empty() {
            return source;
        }

        let indent_str = crate::util::indent( indent + 1 );
        let list_indent_str = crate::util::indent( indent + 2 );
        let list = to_source_code_list( & self.ports, & format!( ";\n{}", list_indent_str ) );
        source.push_str( & format!( "{}{} (\n{}{}\n{});\n", indent_str, PORT, list_indent_str,
                list, indent_str ) );
        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vhdl::direction::Direction;

    const PORTS : &'static str = concat!( "    port (\n", "        a : in integer := 0;\n",
        "        b : out std_logic := '0';\n", "        c : inout boolean;\n",
        "        d : buffer positive\n", "    );\n" );

    /**
     * Create a library list with three entries
     */
    #[test]
    fn port_list() {
        let mut generic_list = PortList::new();
        generic_list.add_port( Port::new_with_default( "a", Direction::IN, "integer", "0" ) );
        generic_list.add_port( Port::new_with_default( "b", Direction::OUT, "std_logic", "'0'" ) );
        generic_list.add_port( Port::new( "c", Direction::INOUT, "boolean" ) );
        generic_list.add_port( Port::new( "d", Direction::BUFFER, "positive" ) );

        assert_eq!(
            generic_list.to_source_code( 0 ),
            format!( "{}", PORTS )
        );
    }
}


