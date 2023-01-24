use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct VhdlError {
    message : String
}

impl VhdlError {
    pub fn new( message : & str ) -> VhdlError {
        VhdlError { message : message.to_string() }
    }
}

impl Error for VhdlError {
}

impl fmt::Display for VhdlError {
    fn fmt( & self, f : & mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{}", self.message )
    }
}

