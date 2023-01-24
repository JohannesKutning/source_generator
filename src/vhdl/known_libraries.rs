use crate::vhdl::library_use::LibraryUse;

const IEEE : &'static str = "ieee";
const STD_LOGIC_1164 : &'static str = "std_logic_1164";
const NUMERIC_STD : &'static str = "numeric_std";
const STD_LOGIC : &'static str = "std_logic";
const SIGNED : &'static str = "signed";
const UNSIGNED : &'static str = "unsigned";

pub fn get_known_library_use( data_type : & str ) -> Option< LibraryUse > {
    let mut data_type = data_type.to_string();
    data_type = data_type.replace( " ", "" );
    if data_type.starts_with( STD_LOGIC ) {
        return Some( LibraryUse::new( IEEE, STD_LOGIC_1164 ) );
    }
    if data_type.starts_with( SIGNED ) {
        return Some( LibraryUse::new( IEEE, NUMERIC_STD ) );
    }
    if data_type.starts_with( UNSIGNED ) {
        return Some( LibraryUse::new( IEEE, NUMERIC_STD ) );
    }
    else {
        return None;
    }
}
