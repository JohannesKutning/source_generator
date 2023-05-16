--
--
--
--
library ieee;
    use ieee.std_logic_1164.all;

entity sub is
    port (
        clk : in std_logic;
        rst : in std_logic
    );
begin
end entity sub;

library ieee;
    use ieee.std_logic_1164.all;

entity main is
    port (
        clk : in std_logic;
        rst : in std_logic
    );
begin
end entity main;

architecture struct of main is
begin
    sub_a : entity work.sub
        port map (
            clk => clk,
            rst => rst
        );
    sub_b : entity work.sub
        port map (
            clk => clk,
            rst => rst
        );
end architecture struct;

