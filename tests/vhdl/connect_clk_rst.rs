use std::path::Path;
use std::error::Error;
use source_generator::source_file::SourceFile;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::vhdl::entity::Entity;
use source_generator::vhdl::entity_interface::EntityInterface;
use source_generator::vhdl::architecture::Architecture;
use source_generator::vhdl::instance::Instance;

/**
 * This example creates two submodules sub_a and sub_b within the main module.  All three
 * modules have the clk_rst_interface.json added.  This interface contains a clk and rst input
 * port.  The main entity connects both submoduls to its clk_rst_interface.  This creates the
 * system shown below:
 *
 *      +----------------------------------------------+
 *      |                       main                   |
 *      |                                              |
 * clk--|>---+-------------------+                     |
 *      |    |                   |                     |
 *      |    |  +-------------+  |  +-------------+    |
 *      |    |  |    sub_a    |  |  |    sub_b    |    |
 *      |    |  |             |  |  |             |    |
 *      |    +--|>            |  +--|>            |    |
 *      |       |             |     |             |    |
 *      |       |             |     |             |    |
 *      |       +-------------+     +-------------+    |
 *      |              |                   |           |
 * rst--|--------------+-------------------+           |
 *      |                                              |
 *      +----------------------------------------------+
 */
fn main() -> Result< (), Box< dyn Error > > {
    let interface = EntityInterface::from_file_unnamed(
        Path::new( "tests/vhdl/clk_rst.json" ) )?;
    let main = Entity::with_interface( "main", & interface.clone() );
    let sub = Entity::with_interface( "sub", & interface );
    let mut arch = Architecture::new( "struct", & main );
    arch.add_instance( Instance::from_entity( "sub_a", & sub ) );
    arch.add_instance( Instance::from_entity( "sub_b", & sub ) );
    arch.connect_instance_to_entity( "sub_a" )?;
    arch.connect_instance_to_entity( "sub_b" )?;

    let mut vhdl_file = VhdlFile::new( "tests/vhdl/connect_clk_rst.vhd" );
    vhdl_file.add_entity( sub );
    vhdl_file.add_architecture( arch );
    vhdl_file.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connect_clk_rst() -> Result< (), Box< dyn Error > > {
        main()
    }
}

