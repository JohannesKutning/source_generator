use crate::element::Element;
use std::fs;

pub trait SourceFile : Element {
    fn write( & self ) -> std::io::Result<()> {
        let mut source = String::new();
        source.push_str( & self.get_file_header() );
        source.push_str( & self.to_source_code( 0 ) );
        fs::write( & self.get_file_name(), & source.into_bytes() )?;
        return Ok(())
    }

    fn get_file_header( & self ) -> String;

    fn get_file_name( & self ) -> & String;
}
