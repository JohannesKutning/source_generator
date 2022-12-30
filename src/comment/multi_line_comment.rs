use crate::element::Element;

#[derive(Debug)]
pub struct MultiLineComment {
    start : String,
    line : String,
    end : String,
    text : String
}

impl MultiLineComment {
    pub fn from( start : & str, line : & str, end : & str, text : & str ) -> MultiLineComment {
        MultiLineComment {
            start : String::from( start ),
            line : String::from( line ),
            end : String::from( end ),
            text : String::from( text )
        }
    }

    pub fn add( & mut self, text : String ) {
        self.text.push_str( & text );
    }
}

impl Element for MultiLineComment {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::from( "" );

        if self.text.is_empty() {
            return source;
        }

        let indent_str = crate::util::indent( indent );
        let split = self.text.split( '\n' );
        let mut start = & self.start;

        /* A comment with more than one line consists of:
         *
         * start text[ 0 ]
         * line text[ 1 ]
         * ...
         * line text[ n ]
         * end
         */
        for i in split {
            if i.len() == 0 {
                source.push_str( & format!( "{}{}\n", & indent_str, & start ) );
            }
            else {
                source.push_str( & format!( "{}{} {}\n", & indent_str, & start, & i ) );
            }
            start = & self.line;
        }
        source.push_str( & format!( "{}{}\n", & indent_str, & self.end ) );

        return source;
    }
}

impl std::fmt::Display for MultiLineComment {
    fn fmt( & self, f : & mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!( f, "{}", self.to_source_code( 0 ) )
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create a simple multi line C/C++ doxygen comment with a single line of text and one level of
     * indentation.
     */
    #[test]
    fn multi_line_single() {
        let c1 = MultiLineComment::from(
            "/**",
            " *",
            "*/",
            "Single line of text"
        );
        assert_eq!(
            c1.to_source_code( 1 ),
            String::from( "    /** Single line of text\n    */\n" )
        );
    }

    /**
     * Create a simple multi line C/C++ doxygen comment with multiple lines of text and two levels
     * of indentation.
     */
    #[test]
    fn multi_line() {
        let c1 = MultiLineComment::from(
            "/**",
            " *",
            " */",
            "First line of text\nsecond line\n\nfourth line"
        );

        assert_eq!(
            c1.to_source_code( 2 ),
            String::from(
                concat!(
                    "        /** First line of text\n",
                    "         * second line\n",
                    "         *\n",
                    "         * fourth line\n",
                    "         */\n"
                )
            )
        );
    }
}

