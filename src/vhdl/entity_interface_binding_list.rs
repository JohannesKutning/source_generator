use crate::vhdl::entity::Entity;
use crate::vhdl::entity_interface_binding::EntityInterfaceBinding;

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

    pub fn get_mut_interfaces( & mut self ) -> & mut Vec< EntityInterfaceBinding > {
        & mut self.interfaces
    }
}

