use crate::element::Element;
use crate::vhdl::keywords::*;
use crate::vhdl::sensitivity_list::SensitivityList;
use crate::vhdl::process_declarative_item::ProcessDeclarativeItem;
use crate::vhdl::concurrent_statement::ConcurrentStatement;

pub struct Process {
    label : String,
    sensitivity_list : SensitivityList,
    declarations : Vec< Box< dyn ProcessDeclarativeItem > >,
}

impl Process {
    pub fn new( label : & str ) -> Process {
        Process { label : label.to_string(), sensitivity_list : SensitivityList::new(), declarations : Vec::new() }
    }
}

impl Element for Process {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        source.push_str( & format!( "{}{} : {}", indent_str, self.label, PROCESS ) );
        if ! self.sensitivity_list.is_empty() {
            source.push_str( & format!( "( {} )", self.sensitivity_list.to_source_code( indent + 1 ) ) );
        }
        source.push_str( " is\n" );

        for declaration in & self.declarations {
            source.push_str( & declaration.to_source_code( indent + 1 ) );
        }
        source.push_str( & format!( "{}{}\n", indent_str, BEGIN ) );
        source.push_str( & format!( "{}{} {} {};\n", indent_str, END, PROCESS, self.label ) );
        return source;
    }
}

impl ConcurrentStatement for Process {
}


//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    const HEADER : &'static str = "test : process is\n";
    const BEGIN : &'static str = "begin\n";
    const END : &'static str = "end process test;\n";

    /**
     * Create a process with no label and content.
     */
    #[test]
    fn process_frame() {
        let process = Process::new( "test" );

        assert_eq!(
            process.to_source_code( 0 ),
            format!( "{}{}{}", HEADER, BEGIN, END )
        );
    }
}

