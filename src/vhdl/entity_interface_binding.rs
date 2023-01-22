use crate::vhdl::binding::Binding;
use crate::vhdl::entity_interface::EntityInterface;

pub struct EntityInterfaceBinding {
    name : String,
    class : String,
    generics : Vec< Binding >,
    ports : Vec< Binding >,
}

impl EntityInterfaceBinding {
    pub fn from_entity_interface( interface : & EntityInterface ) -> EntityInterfaceBinding {
        EntityInterfaceBinding {
                name : interface.get_name().to_string(),
                class : interface.get_class().to_string(),
                generics : EntityInterfaceBinding::generic_bindings_from_interface( interface ),
                ports : EntityInterfaceBinding::port_bindings_from_interface( interface ) }
    }

    pub fn get_name( & self ) -> & String {
        & self.name
    }

    pub fn get_class( & self ) -> & String {
        & self.class
    }

    pub fn get_generics( & self ) -> & Vec< Binding > {
        & self.generics
    }

    pub fn get_ports( & self ) -> & Vec< Binding > {
        & self.ports
    }

    pub fn connect_to_entity_interface( & mut self, entity : & EntityInterface ) {
        let mut idx = 0;
        for generic in & mut self.generics {
            generic.connect_to_generic( & entity.get_generics()[ idx ] );
            idx += 1;
        }
        idx = 0;
        for port in & mut self.ports {
            port.connect_to_port( & entity.get_ports()[ idx ] );
            idx += 1;
        }
    }

    fn generic_bindings_from_interface( interface : & EntityInterface ) -> Vec< Binding > {
        let mut bindings : Vec< Binding > = Vec::new();
        for generic in interface.get_generics() {
            bindings.push( Binding::from_generic( generic ) );
        }
        return bindings;
    }

    fn port_bindings_from_interface( interface : & EntityInterface ) -> Vec< Binding > {
        let mut bindings : Vec< Binding > = Vec::new();
        for port in interface.get_ports() {
            bindings.push( Binding::from_port( port ) );
        }
        return bindings;
    }
}

//------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::path::Path;
    use crate::vhdl::entity::Entity;

    const UNBOUND : &'static str = concat!( "interface test\n",
            "    A integer \n",
            "    B std_logic \n",
            "    C boolean \n",
            "    D positive \n",
            "    a in integer \n",
            "    b out std_logic \n",
            "    c inout boolean \n",
            "    d buffer positive \n" );

    const BOUND : &'static str = concat!( "interface test\n",
            "    A integer A\n",
            "    B std_logic B\n",
            "    C boolean C\n",
            "    D positive D\n",
            "    a in integer a\n",
            "    b out std_logic b\n",
            "    c inout boolean c\n",
            "    d buffer positive d\n" );

    #[test]
    fn from_entity_interface() -> Result< (), Box< dyn Error > > {
        let interface = EntityInterface::from_file_unnamed(
                Path::new( "tests/vhdl/interface.json" ) )?;
        let binding = EntityInterfaceBinding::from_entity_interface( & interface );

        assert_eq!( UNBOUND, to_string( & binding ) );
        Ok(())
    }

    #[test]
    fn connect_to_entity_interface() -> Result< (), Box< dyn Error > > {
        let interface = EntityInterface::from_file_unnamed(
                Path::new( "tests/vhdl/interface.json" ) )?;
        let mut binding = EntityInterfaceBinding::from_entity_interface( & interface );
        let entity = Entity::with_interface( "test", interface );
        binding.connect_to_entity_interface( & entity.get_interfaces()[ 0 ] );
        assert_eq!( BOUND, to_string( & binding ) );
        Ok(())
    }


    fn to_string( binding : & EntityInterfaceBinding ) -> String {
        let mut s = String::new();
        s.push_str( & format!( "{} {}\n", binding.get_name(), binding.get_class() ) );
        for generic in binding.get_generics() {
            s.push_str( & format!( "    {} {} {}\n", generic.get_inner(),
                    generic.get_data_type(), generic.get_outer() ) );
        }
        for port in binding.get_ports() {
            s.push_str( & format!( "    {} {} {} {}\n", port.get_inner(),
                    port.get_direction(), port.get_data_type(), port.get_outer() ) );
        }
        return s;
    }
}

