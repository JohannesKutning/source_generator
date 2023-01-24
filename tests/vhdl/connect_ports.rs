use std::error::Error;
use source_generator::source_file::SourceFile;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::vhdl::entity::Entity;
use source_generator::vhdl::architecture::Architecture;
use source_generator::vhdl::instance::Instance;
use source_generator::vhdl::port::Port;
use source_generator::vhdl::direction::Direction;
use source_generator::vhdl::signal_assignment::SignalAssignment;

/**
 * This example creates two submodules f_AND and f_OR within the main module.  The main module has
 * three input signals a, b and c and one output signal y.  The AND module implements a logical and
 * operation and uses a and b as inputs.  The result y is connected to the signal y_and of the main
 * module.  The OR module takes y_and and c as inputs and performs a logical operation.  Its output
 * y connected to the main module output y.
 *
 *      +--------------------------------+
 *      |              main              |
 *      |                                |
 *      |   +-------+                    |
 *      |   |       |                    |
 * a----|---| f_AND |                    |
 *      |   |       |       +-------+    |
 *      |   |       | y_and |       |    |
 * b----|---|       |-------| f_OR  |    |
 *      |   |       |       |       |    |
 *      |   +-------+       |       |    |
 * c----|-------------------|       |----|---y
 *      |                   |       |    |
 *      |                   +-------+    |
 *      |                                |
 *      +--------------------------------+
 */

fn main() -> Result< (), Box< dyn Error > > {
    let mut main = Entity::new( "main" );
    main.add_port( Port::new( "a", Direction::IN, "std_logic" ) );
    main.add_port( Port::new( "b", Direction::IN, "std_logic" ) );
    main.add_port( Port::new( "c", Direction::IN, "std_logic" ) );
    main.add_port( Port::new( "y", Direction::OUT, "std_logic" ) );

    let mut f_and = Entity::new( "f_AND" );
    f_and.add_port( Port::new( "a", Direction::IN, "std_logic" ) );
    f_and.add_port( Port::new( "b", Direction::IN, "std_logic" ) );
    f_and.add_port( Port::new( "y", Direction::OUT, "std_logic" ) );

    let mut f_and_arch = Architecture::new( "rtl", & f_and );
    f_and_arch.add_signal_assignment( SignalAssignment::new( "y", "a and b" ));

    let mut f_or = Entity::new( "f_OR" );
    f_or.add_port( Port::new( "a", Direction::IN, "std_logic" ) );
    f_or.add_port( Port::new( "b", Direction::IN, "std_logic" ) );
    f_or.add_port( Port::new( "y", Direction::OUT, "std_logic" ) );

    let mut f_or_arch = Architecture::new( "rtl", & f_or );
    f_or_arch.add_signal_assignment( SignalAssignment::new( "y", "a or b" ));

    let mut main_arch = Architecture::new( "struct", & main );
    main_arch.add_instance( Instance::from_entity( "u_f_add", & f_and ) );
    main_arch.add_instance( Instance::from_entity( "u_f_or", & f_or ) );
    main_arch.connect_instance_to_port_by_name( "u_f_add", "a", "a" )?;
    main_arch.connect_instance_to_port_by_name( "u_f_add", "b", "b" )?;
    main_arch.connect_instance_to_signal_by_name( "u_f_add", "y", "y_and" )?;
    main_arch.connect_instance_to_signal_by_name( "u_f_or", "a", "y_and" )?;
    main_arch.connect_instance_to_port_by_name( "u_f_or", "b", "c" )?;
    main_arch.connect_instance_to_port_by_name( "u_f_or", "y", "y" )?;

    let mut vhdl_file = VhdlFile::new( "tests/vhdl/connect_ports.vhd" );
    vhdl_file.add_architecture( f_and_arch );
    vhdl_file.add_architecture( f_or_arch );
    vhdl_file.add_architecture( main_arch );
    vhdl_file.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn connect_ports() -> Result< (), Box< dyn Error > > {
        main()
    }
}

