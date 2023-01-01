use crate::source_file::SourceFile;
use crate::element::Element;
use crate::vhdl::design_unit::DesignUnit;
use crate::vhdl::entity::Entity;
use crate::vhdl::architecture::Architecture;
use crate::vhdl::single_line_comment::SingleLineComment;

pub struct VhdlFile {
    file_name : String,
    design_units : Vec< Box< dyn DesignUnit > >
}

impl VhdlFile {
    pub fn new( file_name : & str ) -> VhdlFile {
        VhdlFile { file_name : file_name.to_string(), design_units : Vec::new() }
    }

    pub fn add_entity( & mut self, design_unit : Entity ) {
        self.design_units.push( Box::< Entity >::new( design_unit ) );
    }

    pub fn add_architecture( & mut self, design_unit : Architecture ) {
        self.design_units.push( Box::< Architecture >::new( design_unit ) );
    }
}

impl SourceFile for VhdlFile {
    fn get_file_header( & self ) -> String {
        SingleLineComment::new_with_text( & crate::util::header() ).to_source_code( 0 )
    }

    fn get_file_name( & self ) -> & String {
        & self.file_name
    }
}

impl Element for VhdlFile {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        for design_unit in & self.design_units {
            source.push_str( & design_unit.to_source_code( indent ) );
            source.push_str( "\n" );
        }
        return source;
    }
}

