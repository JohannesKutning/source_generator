# source_generator
A library with basic source code generation features for application with the
need of source code generation.

## VHDL Generation

The VHDL source code generation is oriented on the reverse BNF of VHDL 2008
with some simplifications.
The example **generate_vhdl_adder.rs** shows the creation of a simple VHDL adder
design.

```rust
fn main() -> Result< (), std::io::Error > {
    let mut entity = Entity::new( "adder" );
    entity.add_library_use( LibraryUse::new( "ieee", "numeric_std" ) );
    entity.add_generic( Generic::new_with_default( "SIZE", "positive", "32" ) );
    entity.add_port( Port::new( "a", Direction::IN, "unsigned( SIZE - 1 downto 0 )" ) );
    entity.add_port( Port::new( "b", Direction::IN, "unsigned( SIZE - 1 downto 0 )" ) );
    entity.add_port( Port::new( "c", Direction::OUT, "unsigned( SIZE - 1 downto 0 )" ) );

    let mut architecture = Architecture::new( "rtl", "adder" );
    architecture.add_signal_assignment( SignalAssignment::new_with_label( "add", "c", "a + b" ));

    let mut vhdl_file = VhdlFile::new( "examples/adder.vhd" );
    vhdl_file.add_entity( entity );
    vhdl_file.add_architecture( architecture );

    vhdl_file.write()?;
}
```

This design contains an entity with two input ports and one output port.
The generated architecture uses an addition of both inputs and writes the
result to the output.

Based on this code generation program the follwoing VHDL code is created.

```vhdl
library ieee;
    use ieee.numeric_std.all;

entity adder is
    generic (
        SIZE : positive := 32
    );
    port (
        a : in unsigned( SIZE - 1 downto 0 );
        b : in unsigned( SIZE - 1 downto 0 );
        c : out unsigned( SIZE - 1 downto 0 )
    );
begin
end entity adder;

architecture rtl of adder is
begin
    add: c <= a + b;
end architecture rtl;
```

