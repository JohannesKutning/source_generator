use std::collections::HashSet;
use crate::element::Element;
use crate::vhdl::constant_declaration::ConstantDeclaration;
use crate::vhdl::signal_declaration::SignalDeclaraion;

enum BlockDeclarationType {
    Signal( usize ),
    Constant( usize ),
}

pub struct BlockDeclarativeList {
    order : Vec< BlockDeclarationType >,
    identifiers : HashSet< String >,
    constants : Vec< ConstantDeclaration >,
    signals : Vec< SignalDeclaraion >
}

impl BlockDeclarativeList {
    pub fn new() -> BlockDeclarativeList {
        BlockDeclarativeList { order : Vec::new(), identifiers : HashSet::new(),
                constants : Vec::new(), signals : Vec::new() }
    }

    pub fn add_signal( & mut self, signal : & SignalDeclaraion ) {
        if ! self.identifiers.contains( signal.get_name() ) {
            self.order.push( BlockDeclarationType::Signal( self.signals.len() ) );
            self.identifiers.insert( signal.get_name().clone() );
            self.signals.push( signal.clone() );
        }
    }

    pub fn add_constant( & mut self, constant : & ConstantDeclaration ) {
        if ! self.identifiers.contains( constant.get_name() ) {
            self.order.push( BlockDeclarationType::Constant( self.constants.len() ) );
            self.identifiers.insert( constant.get_name().clone() );
            self.constants.push( constant.clone() );
        }
    }

    pub fn contains_signal_by_name( & self, name : & str ) -> bool {
        self.signals.iter().any( | s | s.get_name() == name )
    }

    pub fn get_constants( & self ) -> & Vec< ConstantDeclaration > {
        & self.constants
    }
}

impl Element for BlockDeclarativeList {

    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        for entry in & self.order {
            match entry {
                BlockDeclarationType::Signal( idx ) => {
                    source.push_str( & self.signals[ *idx ].to_source_code( indent ) );
                }
                BlockDeclarationType::Constant( idx ) => {
                    source.push_str( & self.constants[ *idx ].to_source_code( indent ) );
                }
            };
        }
        return source;
    }
}
