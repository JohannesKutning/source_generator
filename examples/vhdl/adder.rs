use source_generator::source_file::SourceFile;
use source_generator::vhdl::vhdl_file::VhdlFile;
use source_generator::vhdl::entity::Entity;
use source_generator::vhdl::library_use::LibraryUse;
use source_generator::vhdl::generic::Generic;
use source_generator::vhdl::port::Port;
use source_generator::vhdl::direction::Direction;
use source_generator::vhdl::architecture::Architecture;
use source_generator::vhdl::signal_assignment::SignalAssignment;

fn main() -> Result< (), std::io::Error > {
    let mut adder = Entity::new( "adder" );
    adder.add_library_use( LibraryUse::new( "ieee", "numeric_std" ) );
    adder.add_generic( Generic::new_with_default( "SIZE", "positive", "32" ) );
    adder.add_port( Port::new( "a", Direction::IN, "unsigned( SIZE - 1 downto 0 )" ) );
    adder.add_port( Port::new( "b", Direction::IN, "unsigned( SIZE - 1 downto 0 )" ) );
    adder.add_port( Port::new( "c", Direction::OUT, "unsigned( SIZE - 1 downto 0 )" ) );

    let mut rtl = Architecture::new( "rtl", adder );
    rtl.add_signal_assignment( SignalAssignment::new_with_label( "add", "c", "a + b" ));

    let mut vhdl_file = VhdlFile::new( "examples/vhdl/adder.vhd" );
    vhdl_file.add_architecture( rtl );

    vhdl_file.write()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_vhdl_adder() -> Result< (), std::io::Error > {
        main()
    }
}

