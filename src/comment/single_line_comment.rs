use crate::element::Element;

#[derive(Debug)]
pub struct SingleLineComment {
    start : String,
    text : String
}

impl SingleLineComment
{
    pub fn from( start : & str, text : & str ) -> SingleLineComment {
        SingleLineComment {
            start : String::from( start ),
            text : String::from( text )
        }
    }

    pub fn add( & mut self, text : String )
    {
        self.text.push_str( & text );
    }

    pub fn is_empty( & self ) -> bool {
        self.text.is_empty()
    }
}

impl Element for SingleLineComment {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::from( "" );

        if self.text.is_empty() {
            return source;
        }

        let split = self.text.split( '\n' );
        let indent_str = crate::util::indent( indent );

        for i in split {
            if i.len() == 0 {
                source.push_str( & format!( "{}{}\n", indent_str, & self.start ) );
            }
            else {
                source.push_str( & format!( "{}{} {}\n", indent_str, & self.start, i ) );
            }
        }
        return source;
    }
}

impl std::fmt::Display for SingleLineComment {
    fn fmt( & self, f : & mut std::fmt::Formatter<'_> ) -> std::fmt::Result {
        write!( f, "{}", self.source( 0 ) )
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /**
     * Create a simple single line C/C++ doxygen comment with one level of
     * indentation.
     */
    #[test]
    fn single_line() {
        let c1 = SingleLineComment::from(
            "//!",
            "Single line comment"
        );

        assert_eq!(
            c1.source( 1 ),
            String::from( "    //! Single line comment\n" )
        );
    }

    /**
     * Create a single line Makefile comment block.
     */
    #[test]
    fn single_makefile() {
        let c1 = SingleLineComment::from(
            "#",
            "A single line Makefile comment"
        );

        assert_eq!(
            c1.source( 0 ),
            String::from( "# A single line Makefile comment\n" )
        );
    }

    /**
     * Create a single line C/C++ doxygen comment block that stretches over four lines.
     */
    #[test]
    fn single_line_multi() {
        let c1 = SingleLineComment::from(
            "//!",
            "First line\nsecond line\n\n fourth line"
        );

        assert_eq!(
            c1.source( 0 ),
            String::from(
                concat!(
                    "//! First line\n",
                    "//! second line\n",
                    "//!\n",
                    "//!  fourth line\n"
                )
            )
        );
    }

    /**
     * Create a single line VHDL doxygen comment block that stretches over four lines.
     */
    #[test]
    fn single_vhdl() {
        let c1 = SingleLineComment::from(
            "--!",
            "First line\nsecond line\n\n fourth line"
        );

        assert_eq!(
            c1.source( 0 ),
            String::from(
                concat!(
                    "--! First line\n",
                    "--! second line\n",
                    "--!\n",
                    "--!  fourth line\n"
                )
            )
        );
    }
}


