use std::path::Path;
use std::error::Error;
use source_generator::source_file::SourceFile;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::vhdl::entity::Entity;
use source_generator::vhdl::entity_interface::EntityInterface;
use source_generator::vhdl::architecture::Architecture;
use source_generator::vhdl::instance::Instance;

/**
 *
 *                +---------------------------------------------------------------------------+
 *                |                       main                                                |
 * ADDR_WIDTH-----|----------------------+-----------------------------------------+          |
 * DATA_WIDTH-----|-----------------+----|------------------------------------+    |          |
 * BYTE_COUNT-----|------------+----|----|-------------------------------+    |    |          |
 *                |            |    |    |                               |    |    |          |
 *                |            v    v    v                               v    v    v          |
 *                |  +--------------------------+               +--------------------------+  |
 *                |  |          sub_left        |               |          sub_right       |  |
 *                |  |                          |               |                          |  |
 * avalon_host====|=>|avalon_agent   avalon_host|<= avalon_mm =>|avalon_agent   avalon_host|=>|===>avalon_agent
 *                |  |                          |               |                          |  |
 *                |  +--------------------------+               +--------------------------+  |
 *                |                                                                           |
 *                +---------------------------------------------------------------------------+
 */
fn main() -> Result< (), Box< dyn Error > > {
    let mut host = EntityInterface::from_file( "host",
        Path::new( "tests/vhdl/avalon_mm.json" ) )?;
    host.remove_interface_from_generics();
    let mut agent = host.clone_inverted_with_rename( "agent" );
    agent.remove_generics();
    let mut main = Entity::new( "main" );
    main.add_interface( & host )?;
    main.add_interface( & agent )?;
    let mut sub = Entity::new( "sub" );
    sub.add_interface( & host )?;
    sub.add_interface( & agent )?;
    let mut arch = Architecture::new( "struct", & main );
    arch.add_instance( Instance::from_entity( "left", & sub ) );
    arch.add_instance( Instance::from_entity( "right", & sub ) );
    arch.connect_instance_to_instance_by_interface( "left", "right", "host", "agent" )?;
    arch.connect_instance_to_entity( "left" )?;
    arch.connect_instance_to_entity( "right" )?;
    arch.connect_instance_unbound_by_name( "left" )?;
    arch.connect_instance_unbound_by_name( "right" )?;

    let mut vhdl_file = VhdlFile::new( "tests/vhdl/connect_instances_and_ports.vhd" );
    vhdl_file.add_entity( sub );
    vhdl_file.add_architecture( arch );
    vhdl_file.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn connect_instances_and_ports() -> Result< (), Box< dyn Error > > {
        {
            main()?;
            let actual = read_to_string( "tests/vhdl/connect_instances_and_ports.vhd" )?;
            let expected = read_to_string( "tests/vhdl/expected_connect_instances_and_ports.vhd" )?;
            let actual_lines : Vec< _ > = actual.split( "\n" ).collect();
            let expected_lines : Vec< _ > = expected.split( "\n" ).collect();
            // Skip line 2 containing the generation date and time
            for i in 3..expected_lines.len() {
                assert_eq!( actual_lines[ i ],
                        expected_lines[ i ], " line {}", i + 1 );
            }
        }
        Ok(())
    }
}


