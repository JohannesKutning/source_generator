use chrono;

pub fn indent( level : usize ) -> String {
    return " ".repeat( level * 4 );
}

pub fn header() -> String {
    return format!(
        "\nAutomatically generated on {}.\nDo _NOT_ edit!\n",
        chrono::offset::Local::now().format( "%Y-%m-%d %H:%M:%S" )
    );
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #[test]
    fn indent_zero() {
        let i0 = super::indent( 0 );
        assert_eq!( i0, "" );
    }

    #[test]
    fn indent_one() {
        let i1 = super::indent( 1 );
        assert_eq!( i1, "    " );
    }

    #[test]
    fn indent_four() {
        let i4 = super::indent( 4 );
        assert_eq!( i4, "                " );
    }

    /**
     * Calls the static header creation method and compares it with a locally
     * created header string.
     *
     * The time stamp comparison is a bit ugly and might fail if header and now
     * are called at the edge of a second increment.
     */
    #[test]
    fn header() {
        let header = super::header();
        let now    = chrono::offset::Local::now().format( "%Y-%m-%d %H:%M:%S" );

        let expected = format!(
            "\nAutomatically generated on {}.\nDo _NOT_ edit!\n",
            now
        );

        assert_eq!( header, expected );
    }
}

