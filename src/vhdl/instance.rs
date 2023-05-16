use crate::element::Element;
use crate::vhdl::vhdl_error::VhdlError;
use crate::element::to_source_code_list;
use crate::vhdl::concurrent_statement::ConcurrentStatement;
use crate::vhdl::entity::Entity;
use crate::vhdl::entity_interface::EntityInterface;
use crate::vhdl::entity_interface_binding_list::EntityInterfaceBindingList;
use crate::vhdl::entity_interface_binding::EntityInterfaceBinding;
use crate::vhdl::generic_binding::GenericBinding;
use crate::vhdl::keywords::*;
use crate::vhdl::match_index::*;
use crate::vhdl::signal_declaration::SignalDeclaraion;

#[derive(Clone)]
pub struct Instance {
    name : String,
    library : String,
    entity : String,
    bindings : EntityInterfaceBindingList,
}

impl Instance {
    pub fn from_entity( name : & str, entity : & Entity ) -> Instance {
        Instance { name : name.to_string(),
                library : entity.get_target_library().to_string(),
                entity : entity.get_name().to_string(),
                bindings : EntityInterfaceBindingList::from_entity( entity ) }
    }

    pub fn connect_to_entity( & mut self, entity : & Entity ) {
        let entity_interfaces = entity.get_interfaces();
        let mut unbound_entity_interfaces = vec![ true; entity_interfaces.len() ];
        for instance_interface in self.bindings.get_interfaces_mut() {
            let mut match_index = MatchIndex::new();
            for ( entity_idx, entity_interface ) in entity_interfaces.iter().enumerate() {
                if ! unbound_entity_interfaces[ entity_idx ] {
                    continue;
                }
                match_index.update( entity_idx, Instance::get_match_strength(
                        instance_interface, entity_interface ) );
            }
            if match_index.is_match() {
                instance_interface.connect_to_entity_interface( & entity_interfaces[ match_index.position() ] );
                unbound_entity_interfaces[ match_index.position() ] = false;
            }
        }
    }

    pub fn connect_interface_by_name_to_signal_list( & mut self, name : & str, signal_list : & Vec< SignalDeclaraion > ) {
        let interface = self.bindings.get_interface_by_name_mut( name ).unwrap();
        interface.connect_to_signal_list( signal_list );
    }

    pub fn connect_interface_by_index_to_signal_list( & mut self, index : usize, signal_list : & Vec< SignalDeclaraion > ) {
        self.bindings.get_interfaces_mut()[ index ].connect_to_signal_list( signal_list );
    }

    pub fn connect_generic( & mut self, inner : & str, outer : & str ) -> Result< (), VhdlError > {
        let binding = self.bindings.get_generic_mut( inner )?;
        binding.connect_by_name( outer );
        Ok(())
    }

    pub fn connect_to_port( & mut self, inner : & str, outer : & str ) -> Result< (), VhdlError > {
        let binding = self.bindings.get_port_mut( inner )?;
        binding.connect_by_name( outer );
        Ok(())
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_interfaces( & self ) -> & Vec< EntityInterfaceBinding > {
        & self.bindings.get_interfaces()
    }

    pub fn get_interface_by_name( & self, name : & str ) -> Option< & EntityInterfaceBinding > {
        self.bindings.get_interface_by_name( name )
    }

    pub fn get_instance_interface_matches( & self, inst_b : & Instance ) -> Vec< ( usize, usize ) > {
        let mut matches = Vec::new();
        for ( idx_a, interface_a ) in self.get_interfaces().iter().enumerate() {
            let mut match_index = MatchIndex::new();
            for ( idx_b, interface_b ) in inst_b.get_interfaces().iter().enumerate() {
                match_index.update( idx_b, interface_a.get_instance_matching( interface_b ) );
            }
            if match_index.is_match() {
                matches.push( ( idx_a, match_index.position() ) );
            }
        }
        return matches;
    }

    pub fn get_unbound_generics( & self ) -> Vec< GenericBinding > {
        let mut generics : Vec< GenericBinding > = Vec::new();
        for interface in self.bindings.get_interfaces() {
            for generic in interface.get_unbound_generics()  {
                generics.push( generic );
            }
        }
        return generics;
    }

    pub fn get_port_data_type_by_name( & self, name : & str ) -> Option< & String > {
        for interface in self.bindings.get_interfaces() {
            for port in interface.get_ports() {
                if port.get_inner() == name {
                    return Some( port.get_data_type() )
                }
            }
        }
        None
    }

    fn get_generic_bindings( & self ) -> Vec< Box::< dyn Element > > {
        let mut bindings : Vec< Box< dyn Element > > = Vec::new();
        for interface in self.bindings.get_interfaces() {
            for generic in interface.get_generics() {
                if generic.is_bound() || generic.requires_binding() {
                    bindings.push( Box::new( ( * generic ).clone() ) );
                }
            }
        }
        return bindings;
    }

    fn get_port_bindings( & self ) -> Vec< Box::< dyn Element > > {
        let mut bindings : Vec< Box< dyn Element > > = Vec::new();
        for interface in self.bindings.get_interfaces() {
            for port in interface.get_ports() {
                bindings.push( Box::new( ( * port ).clone() ) );
            }
        }
        return bindings;
    }

    fn get_match_strength( instance : & EntityInterfaceBinding, entity : & EntityInterface )
            -> u32 {
        if instance.is_bound() {
            return NONE;
        }
        let instance_name = instance.get_name().to_string().to_lowercase();
        let entity_name = entity.get_name().to_string().to_lowercase();
        let class_match : bool = instance.get_class() == entity.get_class();
        let name_match : bool = instance_name == entity_name;
        let instance_in_entity = entity_name.contains( & instance_name );
        let entity_in_instance = instance_name.contains( & entity_name );
        if class_match && name_match {
            return FULL;
        }
        else if instance_in_entity || entity_in_instance {
            return PARTIAL;
        }
        else if class_match {
            return CLASS;
        }
        else {
            return NONE;
        }
    }

    pub fn contains_interface( & self, name : & str ) -> bool {
        self.bindings.contains_interface( name )
    }
}

impl Element for Instance {
    fn to_source_code( & self, indent : usize ) -> String {
        let mut source = String::new();
        let indent_str = crate::util::indent( indent );
        let map_indent_str = crate::util::indent( indent + 1 );
        let binding_indent_str = crate::util::indent( indent + 2 );
        source.push_str( & format!( "{}{} : {} {}.{}\n", indent_str, self.name, ENTITY,
                self.library, self.entity ) );

        let generic_bindings : Vec< Box< dyn Element > > = self.get_generic_bindings();
        let has_generic_bindings : bool = ! generic_bindings.is_empty();
        if has_generic_bindings {
            source.push_str( & format!( "{}{} {} (\n", map_indent_str, GENERIC, MAP ) );
            source.push_str( & format!( "{}{}\n", binding_indent_str,
                    to_source_code_list( & generic_bindings, & format!( ",\n{}", binding_indent_str ) ) ) );
            source.push_str( & format!( "{})", map_indent_str ) );
        }
        let port_bindings : Vec< Box< dyn Element > > = self.get_port_bindings();
        let has_port_bindings : bool = ! port_bindings.is_empty();
        if has_port_bindings {
            if has_generic_bindings {
                source.push_str( "\n" );
            }
            source.push_str( & format!( "{}{} {} (\n", map_indent_str, PORT, MAP ) );
            source.push_str( & format!( "{}{}\n", binding_indent_str,
                    to_source_code_list( & port_bindings, & format!( ",\n{}", binding_indent_str ) ) ) );
            source.push_str( & format!( "{})", map_indent_str ) );
        }
        source.push_str( ";\n" );
        return source;
    }
}

impl ConcurrentStatement for Instance {
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
}

