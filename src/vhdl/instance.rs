use crate::element::Element;
use crate::element::to_source_code_list;
use crate::vhdl::concurrent_statement::ConcurrentStatement;
use crate::vhdl::entity::Entity;
use crate::vhdl::entity_interface::EntityInterface;
use crate::vhdl::entity_interface_binding_list::EntityInterfaceBindingList;
use crate::vhdl::entity_interface_binding::EntityInterfaceBinding;
use crate::vhdl::keywords::*;

pub struct Instance {
    name : String,
    library : String,
    entity : String,
    bindings : EntityInterfaceBindingList,
}

struct Match {
    pub index : usize,
    pub strength : u32,
}

impl Match {
    pub fn new() -> Match {
        Match { index : 0, strength : 0 }
    }

    pub fn update( & mut self, index : usize, strength : u32 ) {
        if self.strength < strength {
            self.index = index;
            self.strength = strength;
        }
    }
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
        for instance_interface in self.bindings.get_mut_interfaces() {
            let mut m = Match::new();
            for ( entity_idx, entity_interface ) in entity_interfaces.iter().enumerate() {
                if ! unbound_entity_interfaces[ entity_idx ] {
                    continue;
                }
                m.update( entity_idx,
                        Instance::get_match_strength( instance_interface, entity_interface ) );
            }
            if m.strength > 0 {
                instance_interface.connect_to_entity_interface( & entity_interfaces[ m.index ] );
                unbound_entity_interfaces[ m.index ] = false;
            }
        }
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    fn get_generic_bindings( & self ) -> Vec< Box::< dyn Element > > {
        let mut bindings : Vec< Box< dyn Element > > = Vec::new();
        for interface in self.bindings.get_interfaces() {
            for generic in interface.get_generics() {
                bindings.push( Box::new( ( * generic ).clone() ) );
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
        let instance_name = instance.get_name().to_string().to_lowercase();
        let entity_name = entity.get_name().to_string().to_lowercase();
        let class_match : bool = instance.get_class() == entity.get_class();
        let name_match : bool = instance_name == entity_name;
        let instance_in_entity = entity_name.contains( & instance_name );
        let entity_in_instance = instance_name.contains( & entity_name );
        if class_match && name_match {
            return 3;
        }
        else if instance_in_entity || entity_in_instance {
            return 2;
        }
        else if class_match {
            return 1;
        }
        else {
            return 0;
        }
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
            source.push_str( & format!( "{} )", map_indent_str ) );
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
            source.push_str( & format!( "{} )", map_indent_str ) );
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

