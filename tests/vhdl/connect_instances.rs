use std::path::Path;
use std::error::Error;
use source_generator::source_file::SourceFile;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::vhdl::entity::Entity;
use source_generator::vhdl::entity_interface::EntityInterface;
use source_generator::vhdl::architecture::Architecture;
use source_generator::vhdl::instance::Instance;

/**
 * This example creates the two submodules host and agent within the main module.
 * Module host implements an avlalon_mm host interface and agent implements the
 * corresponding agent.
 *
 *                +-------------------------------------------------+
 *                |                       main                      |
 * ADDR_WIDTH-----|--------------+-----------------------------+    |
 * DATA_WIDTH-----|---------+----|------------------------+    |    |
 * BYTE_COUNT-----|----+----|----|-------------------+    |    |    |
 *                |    |    |    |                   |    |    |    |
 *                |    v    v    v                   v    v    v    |
 *                |  +-------------+               +-------------+  |
 *                |  |     host    |               |    agent    |  |
 *                |  |             |<= avalon_mm =>|             |  |
 *                |  |             |               |             |  |
 *                |  +-------------+               +-------------+  |
 *                |                                                 |
 *                +-------------------------------------------------+
 */
fn main() -> Result< (), Box< dyn Error > > {
    let avalon_mm = EntityInterface::from_file_unnamed(
        Path::new( "tests/vhdl/avalon_mm.json" ) )?;
    let mut main = Entity::new( "main" );
    for generic in avalon_mm.get_generics() {
        main.add_generic( generic.clone() );
    }
    let host = Entity::with_interface( "host", & avalon_mm );
    let agent = Entity::with_interface( "agent", & avalon_mm.clone_inverted() );
    let mut arch = Architecture::new( "struct", & main );
    arch.add_instance( Instance::from_entity( "host", & host ) );
    arch.add_instance( Instance::from_entity( "agent", & agent ) );
    arch.connect_instance_to_instance( "host", "agent" )?;
    arch.connect_instance_unbound_by_name( "host" )?;
    arch.connect_instance_unbound_by_name( "agent" )?;

    let mut vhdl_file = VhdlFile::new( "tests/vhdl/connect_instances.vhd" );
    vhdl_file.add_entity( host );
    vhdl_file.add_entity( agent );
    vhdl_file.add_architecture( arch );
    vhdl_file.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::*;

    #[test]
    fn connect_instances() -> Result< (), Box< dyn Error > > {
        {
            main()?;
            let actual = read_to_string( "tests/vhdl/connect_instances.vhd" )?;
            let expected = read_to_string( "tests/vhdl/expected_connect_instances.vhd" )?;
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


