use std::error::Error;
use linked_hash_map::LinkedHashMap;
use crate::element::Element;
use crate::vhdl::vhdl_error::VhdlError;
use crate::vhdl::keywords::*;
use crate::vhdl::design_unit::DesignUnit;
use crate::vhdl::entity::Entity;
use crate::vhdl::block_declarative_list::BlockDeclarativeList;
use crate::vhdl::constant_declaration::ConstantDeclaration;
use crate::vhdl::signal_declaration::SignalDeclaraion;
use crate::vhdl::concurrent_statement::ConcurrentStatement;
use crate::vhdl::signal_assignment::SignalAssignment;
use crate::vhdl::instance::Instance;
use crate::vhdl::process::Process;
use crate::vhdl::entity_interface_binding::EntityInterfaceBinding;
use crate::vhdl::generic::Generic;
use crate::vhdl::generic_binding::GenericBinding;
use crate::vhdl::match_index::MatchIndex;

pub struct Architecture {
    name : String,
    entity : Entity,
    declarations : BlockDeclarativeList,
    instances : LinkedHashMap< String, Instance >,
    statements : Vec< Box< dyn ConcurrentStatement > >
}

impl Architecture {
    pub fn new( name : & str, entity : & Entity ) -> Architecture {
        Architecture { name : name.to_string(), entity : ( * entity ).clone(),
                declarations : BlockDeclarativeList::new(), instances : LinkedHashMap::new(),
                statements : Vec::new() }
    }

    pub fn add_constant_declaration( & mut self, constant : & ConstantDeclaration ) {
        self.entity.add_missing_library_use( constant.get_data_type() );
        self.declarations.add_constant( constant );
    }

    pub fn add_signal_declaration( & mut self, signal : & SignalDeclaraion ) {
        self.entity.add_missing_library_use( signal.get_data_type() );
        self.declarations.add_signal( signal );
    }

    pub fn add_signal_declaration_list( & mut self, signal_list : & Vec< SignalDeclaraion > ) {
        for signal in signal_list {
            self.add_signal_declaration( signal );
        }
    }

    pub fn add_signal_assignment( & mut self, signal_assignment : SignalAssignment ) {
        self.statements.push( Box::< SignalAssignment >::new( signal_assignment ) );
    }

    pub fn add_instance( & mut self, instance : Instance ) {
        self.instances.insert( instance.get_name().to_string(), instance );
    }

    pub fn add_process( & mut self, process : Process ) {
        self.statements.push( Box::< Process >::new( process ) );
    }

    pub fn connect_instance_to_entity( & mut self, name : & str ) -> Result< (), VhdlError > {
        self.requires_instance( name )?;
        self.instances.get_mut( name ).unwrap().connect_to_entity( & self.entity );
        Ok(())
    }

    pub fn connect_instance_to_instance( & mut self, inst_name_a : & str, inst_name_b: & str )
            -> Result< (), VhdlError > {
        println!( "connect_instance_to_instance {} {}", inst_name_a, inst_name_b );
        let matches;
        {
            self.requires_instance( inst_name_a )?;
            self.requires_instance( inst_name_b )?;
            let inst_a : & Instance = self.instances.get( inst_name_a ).unwrap();
            let inst_b : & Instance = self.instances.get( inst_name_b ).unwrap();
            // find a list of matching interfaces
            matches = inst_a.get_instance_interface_matches( & inst_b );
        }
        let mut connection_signal_lists : Vec< Vec< SignalDeclaraion > > = Vec::new();
        {
            for ( a, _ ) in & matches {
                let inst : & mut Instance = self.instances.get_mut( inst_name_a ).unwrap();
                let interface_a : & EntityInterfaceBinding = & inst.get_interfaces()[ * a ];
                let signal_list = interface_a.get_connection_signal_list( inst_name_a, inst_name_b );
                inst.connect_interface_by_index_to_signal_list( * a, & signal_list );
                connection_signal_lists.push( signal_list );
            }
        }
        {
            for ( i, ( _, b ) ) in matches.iter().enumerate() {
                let inst : & mut Instance = self.instances.get_mut( inst_name_b ).unwrap();
                inst.connect_interface_by_index_to_signal_list( * b, & connection_signal_lists[ i ] );
            }
        }
        {
            for signal_list in & connection_signal_lists {
                self.add_signal_declaration_list( signal_list );
            }
        }
        Ok(())
    }

    pub fn connect_instance_unbound_by_name( & mut self, instance : & str )
            -> Result< (), VhdlError > {
        println!( "connect_instance_unbound_by_name" );
        self.requires_instance( instance )?;
        let mut matches : Vec< ( String, String ) > = Vec::new();
        {
            let inst : Instance = ( * self.instances.get( instance ).unwrap() ).clone();
            for generic in & inst.get_unbound_generics() {
                let outer = self.get_instance_generic_match( generic );
                matches.push( ( generic.get_inner().clone(), outer ) );
            }
        }
        let inst : & mut Instance = self.instances.get_mut( instance ).unwrap();
        for ( inner, outer ) in matches {
            if ! outer.is_empty() {
                inst.connect_generic( & inner, & outer )?;
            }
        }

        Ok(())
    }

    pub fn connect_instance_to_port_by_name( & mut self, instance : & str, inner : & str,
            outer : & str ) -> Result< (), Box< dyn Error > > {
        let instance : & mut Instance = self.get_instance_mut( instance )?;
        instance.connect_to_port( inner, outer )?;

        Ok(())
    }

    pub fn connect_instance_to_signal_by_name( & mut self, instance : & str, inner : & str,
            outer : & str ) -> Result< (), Box< dyn Error > > {
        {
            let inst : & mut Instance = self.get_instance_mut( instance )?;
            inst.connect_to_port( inner, outer )?;
        }
        {
            let inst : & Instance = self.get_instance( instance )?;
            let data_type = inst.get_port_data_type_by_name( inner ).unwrap().clone();
            self.add_signal_declaration( & SignalDeclaraion::new( outer, & data_type ) );
        }
        Ok(())
    }

    fn get_instance( & self, instance : & str ) -> Result< & Instance, VhdlError > {
        match self.instances.get( instance ) {
            Some( instance ) => Ok( instance ),
            None => Err( VhdlError::new( & format!(
                            "error: Architecture {:?} does not contain instance {:?}", self.name,
                            instance ) ) )
        }
    }

    fn get_instance_mut( & mut self, instance : & str ) -> Result< & mut Instance, VhdlError > {
        match self.instances.get_mut( instance ) {
            Some( instance ) => Ok( instance ),
            None => Err( VhdlError::new( & format!(
                            "error: Architecture {:?} does not contain instance {:?}", self.name,
                            instance ) ) )
        }
    }

    fn requires_instance( & self, name : & str ) -> Result< (), VhdlError > {
        if ! self.instances.contains_key( name ) {
            return Err( VhdlError::new( & format!( "error: Instance {:?} not found in architecture {:?}!",
                    name, self.name ) ) );
        }
        Ok(())
    }

    fn get_instance_generic_match( & self, binding : & GenericBinding ) -> String {
        let constants = self.declarations.get_constants();
        let mut constant_match = MatchIndex::new();
        for ( idx, constant ) in constants.iter().enumerate() {
            constant_match.update( idx,
                    self.get_generic_constant_match_strength( binding, constant) );
        }
        let generics = self.entity.get_generics();
        let mut generic_match = MatchIndex::new();
        for ( idx, generic ) in generics.iter().enumerate() {
            generic_match.update( idx,
                    self.get_generic_generic_match_strength( binding, generic ) );
        }
        // in case both matches are zero constant is not greater than generic
        if constant_match.strength > generic_match.strength {
            return constants[ constant_match.index ].get_name().clone();
        }
        else if generic_match.strength > 0 {
            return generics[ generic_match.index ].get_name().clone();
        }
        return String::new();
    }

    fn get_generic_constant_match_strength( & self, _binding : & GenericBinding,
            _constant : & ConstantDeclaration ) -> u32 {
        return 0;
    }

    fn get_generic_generic_match_strength( & self, binding : & GenericBinding,
            generic : & Generic ) -> u32 {
        let binding_name = binding.get_inner().to_string().to_lowercase();
        let generic_name = generic.get_name().to_string().to_lowercase();
        let data_type_match : bool = binding.get_data_type() == generic.get_data_type();
        let name_match : bool = binding_name == generic_name;
        let binding_in_generic = generic_name.contains( & binding_name );
        let generic_in_binding = binding_name.contains( & generic_name );
        if ! data_type_match {
            return 0;
        }
        if name_match {
            return 3;
        }
        else if binding_in_generic || generic_in_binding {
            return 2;
        }
        return 1;
    }
}

impl Element for Architecture {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = self.entity.to_source_code( indent );
        source.push_str( "\n" );
        let indent_str = crate::util::indent( indent );

        source.push_str( & format!( "{}{} {} {} {} {}\n", indent_str, ARCHITECTURE, self.name, OF,
                self.entity.get_name(), IS ) );
        source.push_str( & self.declarations.to_source_code( indent + 1 ) );
        source.push_str( & format!( "{}{}\n", indent_str, BEGIN ) );
        for ( _name, instance ) in & self.instances {
            source.push_str( & instance.to_source_code( indent + 1 ) );
        }
        for statement in & self.statements {
            source.push_str( & statement.to_source_code( indent + 1 ) );
        }
        source.push_str( & format!( "{}{} {} {};\n", indent_str, END, ARCHITECTURE, self.name ) );

        return source;
    }
}

impl DesignUnit for Architecture {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    const NAME : &'static str = "rtl";
    const ENTITY : &'static str = "test";
    const ENTITY_TEST : &'static str = "entity test is\nbegin\nend entity test;\n\n";
    const HEADER : &'static str = "architecture rtl of test is\n";
    const BEGIN : &'static str = "begin\n";
    const END : &'static str = "end architecture rtl;\n";
    const SIGNAL_DECLARATION : &'static str = "    signal signal_1 : boolean;\n";
    const SIGNAL_ASSIGNMENT : &'static str = "    s1: signal_1 <= true;\n";
    const CONSTANT_DECLARATION : &'static str = "    constant const_1 : integer := 12;\n";

    /**
     * Create a architecture with no content.
     */
    #[test]
    fn architecture_frame() {
        let architecture = Architecture::new( NAME, & Entity::new( ENTITY ) );
        assert_eq!( architecture.to_source_code( 0 ),
            format!( "{}{}{}{}", ENTITY_TEST, HEADER, BEGIN, END ) );
    }

    /**
     * Create a architecture with a singal declaration.
     */
    #[test]
    fn architecture_with_signal_declaration() {
        let mut architecture = Architecture::new( NAME, & Entity::new( ENTITY ) );
        architecture.add_signal_declaration( & SignalDeclaraion::new( "signal_1", "boolean" ) );
        assert_eq!( architecture.to_source_code( 0 ),
            format!( "{}{}{}{}{}", ENTITY_TEST, HEADER, SIGNAL_DECLARATION, BEGIN, END ) );
    }

    /**
     * Create a architecture with a singal declaration and assignment.
     */
    #[test]
    fn architecture_with_signal_declaration_and_assignment() {
        let mut architecture = Architecture::new( NAME, & Entity::new( ENTITY ) );
        architecture.add_signal_declaration( & SignalDeclaraion::new( "signal_1", "boolean" ) );
        architecture.add_signal_assignment( SignalAssignment::new_with_label( "s1", "signal_1", "true" ) );

        assert_eq!( architecture.to_source_code( 0 ),
            format!( "{}{}{}{}{}{}", ENTITY_TEST, HEADER, SIGNAL_DECLARATION, BEGIN, SIGNAL_ASSIGNMENT, END )
        );
    }

    /**
     * Create a architecture with a constant declaration.
     */
    #[test]
    fn architecture_with_constant_declaration_and_assignment() {
        let mut architecture = Architecture::new( NAME, & Entity::new( ENTITY ) );
        architecture.add_constant_declaration( & ConstantDeclaration::new( "const_1", "integer", "12" ) );

        assert_eq!( architecture.to_source_code( 0 ),
            format!( "{}{}{}{}{}", ENTITY_TEST, HEADER, CONSTANT_DECLARATION, BEGIN, END ) );
    }

    /**
     * Create a architecture with a constant declaration.
     */
    #[test]
    fn connect_instance_to_instance() {
        let mut _architecture = Architecture::new( NAME, & Entity::new( ENTITY ) );
    }
}

