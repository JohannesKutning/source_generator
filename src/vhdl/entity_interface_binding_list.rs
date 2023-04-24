use crate::vhdl::entity::Entity;
use crate::vhdl::vhdl_error::VhdlError;
use crate::vhdl::entity_interface_binding::EntityInterfaceBinding;
use crate::vhdl::generic_binding::GenericBinding;
use crate::vhdl::port_binding::PortBinding;

#[derive(Clone)]
pub struct EntityInterfaceBindingList {
    interfaces : Vec< EntityInterfaceBinding >,
}

impl EntityInterfaceBindingList {
    pub fn from_entity( entity : & Entity ) -> EntityInterfaceBindingList {
        let mut interfaces : Vec< EntityInterfaceBinding > = Vec::new();
        for interface in entity.get_interfaces() {
            interfaces.push( EntityInterfaceBinding::from_entity_interface( interface ) );
        }
        EntityInterfaceBindingList { interfaces : interfaces }
    }

    pub fn get_interfaces( & self ) -> & Vec< EntityInterfaceBinding > {
        & self.interfaces
    }

    pub fn get_interfaces_mut( & mut self ) -> & mut Vec< EntityInterfaceBinding > {
        & mut self.interfaces
    }

    pub fn get_generic_mut( & mut self, name : & str ) -> Result< & mut GenericBinding, VhdlError > {
        for interface in & mut self.interfaces {
            if interface.contains_generic( name ) {
                return Ok( interface.get_generic_mut( name ).unwrap() );
            }
        }
        Err( VhdlError::new( & format!( "error: Port {:?} not found!", name ) ) )
    }

    pub fn get_port_mut( & mut self, name : & str ) -> Result< & mut PortBinding, VhdlError > {
        for interface in & mut self.interfaces {
            if interface.contains_port( name ) {
                return Ok( interface.get_port_mut( name ).unwrap() );
            }
        }
        Err( VhdlError::new( & format!( "error: Port {:?} not found!", name ) ) )
    }
}

