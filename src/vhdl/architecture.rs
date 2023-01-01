use crate::element::Element;
use crate::vhdl::keywords::*;
use crate::vhdl::design_unit::DesignUnit;
use crate::vhdl::concurrent_statement::ConcurrentStatement;
use crate::vhdl::signal_assignment::SignalAssignment;

pub struct Architecture {
    name : String,
    entity : String,
    statements : Vec< Box< dyn ConcurrentStatement > >
}

impl Architecture {
    pub fn new( name : & str, entity : & str ) -> Architecture {
        Architecture { name : name.to_string(), entity : entity.to_string(),
                statements : Vec::new() }
    }

    pub fn add_signal_assignment( & mut self, signal_assignment : SignalAssignment ) {
        self.statements.push( Box::< SignalAssignment >::new( signal_assignment ) );
    }
}

impl Element for Architecture {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {} {} {} {}\n", indent_str, ARCHITECTURE, self.name, OF,
                self.entity, IS ) );
        source.push_str( & format!( "{}{}\n", indent_str, BEGIN ) );

        for statement in & self.statements {
            source.push_str( & statement.to_source_code( indent + 1 ) );
        }

        source.push_str( & format!( "{}{} {} {};\n", indent_str, END, ARCHITECTURE, self.name ) );

        return source;
    }
}

impl DesignUnit for Architecture {
}

