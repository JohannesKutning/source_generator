use linked_hash_set::LinkedHashSet;

use crate::element::Element;

pub struct SensitivityList {
    signals : LinkedHashSet< String >
}

impl SensitivityList {
    pub fn new() -> SensitivityList {
        SensitivityList { signals : LinkedHashSet::new() }
    }

    pub fn new_with_signal( signal : & str ) -> SensitivityList {
        let mut signals : LinkedHashSet< String > = LinkedHashSet::new();
        signals.insert( signal.to_string() );
        SensitivityList { signals : signals }
    }

    pub fn new_with_list( list : Vec< String > ) -> SensitivityList {
        let mut signals : LinkedHashSet< String > = LinkedHashSet::new();
        for signal in list {
            signals.insert_if_absent( signal.to_string() );
        }
        SensitivityList { signals : signals }
    }

    pub fn is_empty( & self ) -> bool {
        self.signals.is_empty()
    }

    pub fn add_signal( & mut self, signal : & str ) {
        self.signals.insert_if_absent( signal.to_string() );
    }

    pub fn add_list( & mut self, list : Vec< String > ) {
        for i in & list {
            self.add_signal( i );
        }
    }
}

impl Element for SensitivityList {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        let mut first = true;
        for signal in & self.signals {
            if first {
                source.push_str( & format!( "{}", signal ) );
                first = false;
            }
            else {
                source.push_str( & format!( ",\n{}{}", indent_str, signal ) );
            }

        }
        return source;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create an empty sensitivity list
     */
    #[test]
    fn empty_list() {
        let sensitivity_list = SensitivityList::new();
        assert_eq!(
            sensitivity_list.to_source_code( 0 ),
            String::new()
        );
    }

    /**
     * Create a sensitivity list with all entry
     */
    #[test]
    fn single_list_entry() {
        let sensitivity_list = SensitivityList::new_with_signal( "all" );
        assert_eq!(
            sensitivity_list.to_source_code( 0 ),
            "all".to_string()
        );
    }

    /**
     * Create a sensitivity list with all entry
     */
    #[test]
    fn two_list_entries() {
        let sensitivity_list = SensitivityList::new_with_list( vec![ "a".to_string(), "b".to_string() ] );
        assert_eq!(
            sensitivity_list.to_source_code( 1 ),
            "a,\n    b".to_string()
        );
    }

    /**
     * Create a sensitivity list and add three single signals.
     */
    #[test]
    fn add_three_entries() {
        let mut sensitivity_list = SensitivityList::new();
        sensitivity_list.add_signal( "a" );
        sensitivity_list.add_signal( "b" );
        sensitivity_list.add_signal( "c" );
        assert_eq!(
            sensitivity_list.to_source_code( 2 ),
            "a,\n        b,\n        c".to_string()
        );
    }

    /**
     * Create a sensitivity list and add a list with three signals.
     */
    #[test]
    fn add_list() {
        let mut sensitivity_list = SensitivityList::new();
        sensitivity_list.add_list( vec![ "a".to_string(), "b".to_string(), "c".to_string() ] );
        assert_eq!(
            sensitivity_list.to_source_code( 2 ),
            "a,\n        b,\n        c".to_string()
        );
    }

    /**
     * Create a sensitivity list and add a list with three signals.
     */
    #[test]
    fn add_two_lists() {
        let mut sensitivity_list = SensitivityList::new();
        sensitivity_list.add_list( vec![ "a".to_string(), "b".to_string(), "c".to_string() ] );
        sensitivity_list.add_list( vec![ "b".to_string(), "c".to_string(), "d".to_string() ] );
        assert_eq!(
            sensitivity_list.to_source_code( 2 ),
            "a,\n        b,\n        c,\n        d".to_string()
        );
    }
}

