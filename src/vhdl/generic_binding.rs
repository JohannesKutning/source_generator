use crate::element::Element;
use crate::vhdl::generic::Generic;

#[derive(Clone)]
pub struct GenericBinding {
    inner : String,
    data_type : String,
    outer : String,
    requires_binding : bool
}

impl GenericBinding {
    pub fn from_generic( generic : & Generic ) -> GenericBinding{
        GenericBinding { inner : generic.get_name().to_string(),
                data_type : generic.get_data_type().to_string(),
                outer : String::new(), requires_binding : ! generic.has_default() }
    }

    pub fn connect( & mut self, generic : & Generic ) {
        self.outer = generic.get_name().to_string();
    }

    pub fn connect_by_name( & mut self, outer : & str ) {
        self.outer = outer.to_string();
    }

    pub fn get_inner( & self ) -> & String {
        & self.inner
    }

    pub fn get_data_type( & self ) -> & String {
        & self.data_type
    }

    pub fn get_outer( & self ) -> & String {
        & self.outer
    }

    pub fn is_bound( & self ) -> bool {
        ! self.outer.is_empty()
    }

    pub fn requires_binding( & self ) -> bool {
        self.requires_binding
    }
}

impl Element for GenericBinding {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        if self.outer.is_empty() && ! self.requires_binding {
            return source;
        }
        let indent_str = crate::util::indent( indent );
        source.push_str( & format!( "{}{} => {}", indent_str, self.inner, self.outer ) );
        return source;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_generic() {
        let binding = GenericBinding::from_generic( & Generic::new( "test", "boolean" ) );
        assert_eq!( & binding.to_source_code( 0 ), "test => " );
    }

    #[test]
    fn connect_to_generic() {
        let generic = Generic::new( "extern", "boolean" );
        let mut binding = GenericBinding::from_generic( & Generic::new( "test", "boolean" ) );
        binding.connect( & generic );
        assert_eq!( & binding.to_source_code( 0 ), "test => extern" );
    }
}



