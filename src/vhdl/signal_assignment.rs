use crate::element::Element;
use crate::vhdl::operators::*;
use crate::vhdl::concurrent_statement::ConcurrentStatement;

pub struct SignalAssignment {
    label : String,
    signal : String,
    expression : String
}

impl SignalAssignment {
    pub fn new( signal : & str, expression : & str ) -> SignalAssignment {
        SignalAssignment { label : String::new(), signal : signal.to_string(),
                expression : expression.to_string() }

    }

    pub fn new_with_label( label : & str, signal : & str, expression : & str ) -> SignalAssignment {
        SignalAssignment { label : label.to_string(), signal : signal.to_string(),
                expression : expression.to_string() }
    }
}

impl Element for SignalAssignment {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        let label = match self.label.is_empty() {
            true => String::new(),
            false => format!( "{}:", self.label )
        };
        source.push_str( & format!( "{}{} {} {} {};\n", indent_str, label, self.signal,
                ASSIGN_SIGNAL, self.expression ) );

        return source;
    }
}

impl ConcurrentStatement for SignalAssignment {
}

