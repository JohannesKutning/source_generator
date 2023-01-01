use crate::element::Element;

const START : &'static str = "--";

pub struct SingleLineComment {
    comment : crate::comment::single_line_comment::SingleLineComment
}

impl SingleLineComment {
    pub fn new() -> SingleLineComment {
        SingleLineComment { comment : crate::comment::single_line_comment::SingleLineComment::new( START, "" ) }
    }

    pub fn new_with_text( text : & str ) -> SingleLineComment {
        SingleLineComment { comment : crate::comment::single_line_comment::SingleLineComment::new( START, text ) }
    }

    pub fn is_empty( & self ) -> bool {
        self.comment.is_empty()
    }
}

impl Element for SingleLineComment {
    fn to_source_code( & self, indent : usize ) -> String {
        self.comment.to_source_code( indent )
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create a simple single line VHDL comment with one level of indentation.
     */
    #[test]
    fn single_line() {
        let c1 = SingleLineComment::new_with_text(
            "Single line comment"
        );

        assert_eq!(
            c1.to_source_code( 1 ),
            String::from( "    -- Single line comment\n" )
        );
    }
}

