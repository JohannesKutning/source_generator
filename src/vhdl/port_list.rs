use crate::element::Element;
use crate::vhdl::port::Port;
use crate::vhdl::keywords::PORT;

pub struct PortList {
    ports : Vec< Port >
}

impl PortList {
    pub fn new() -> PortList {
        PortList { ports : Vec::new() }
    }

    pub fn add_port( & mut self, port : Port ) {
        self.ports.push( port );
    }
}

impl Element for PortList {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();

        if self.ports.is_empty() {
            return source;
        }

        let indent_str = crate::util::indent( indent + 1 );
        source.push_str( & format!( "{}{} (\n", indent_str, PORT ) );

        for ( pos, port ) in self.ports.iter().enumerate() {
            source.push_str( & port.to_source_code( indent + 2 ) );
            if pos < self.ports.len() - 1 {
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


