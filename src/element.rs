pub trait Element {
    fn to_source_code( & self, indent : usize ) -> String;
}

pub fn to_source_code_list( vec : & Vec< Box< dyn Element > >, delimiter : & str ) -> String {
    let mut source = String::new();
    for ( pos, element ) in vec.iter().enumerate() {
        source.push_str( & element.to_source_code( 0 ) );
        if pos < vec.len() - 1 {
            source.push_str( delimiter );
        }
    }
    return source;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestElement {
        name : String
    }

    impl TestElement {
        pub fn new( name : & str ) -> TestElement {
            TestElement { name : name.to_string() }
        }
    }

    impl Element for TestElement {
        fn to_source_code( & self, _indent : usize ) -> String {
            self.name.clone()
        }
    }

    #[test]
    fn comma_separated_list() {
        let source = to_source_code_list( & create_test_list(), ", " );
        assert_eq!( source, "1, 2, 3, 4, 5".to_string() );
    }

    #[test]
    fn semicolon_separated_list() {
        let source = to_source_code_list( & create_test_list(), ";\n    " );
        assert_eq!( source, "1;\n    2;\n    3;\n    4;\n    5".to_string() );
    }

    fn create_test_list() -> Vec< Box< dyn Element > > {
        vec![ Box::new( TestElement::new( "1" ) ), Box::new( TestElement::new( "2" ) ),
            Box::new( TestElement::new( "3" ) ), Box::new( TestElement::new( "4" ) ),
                Box::new( TestElement::new( "5" ) ) ]
    }

}

