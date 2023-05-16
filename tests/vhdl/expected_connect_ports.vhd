--
-- Automatically generated on 2023-05-16 22:25:26.
-- Do _NOT_ edit!
--
library ieee;
    use ieee.std_logic_1164.all;

entity f_AND is
    port (
        a : in std_logic;
        b : in std_logic;
        y : out std_logic
    );
begin
end entity f_AND;

architecture rtl of f_AND is
begin
     y <= a and b;
end architecture rtl;

library ieee;
    use ieee.std_logic_1164.all;

entity f_OR is
    port (
        a : in std_logic;
        b : in std_logic;
        y : out std_logic
    );
begin
end entity f_OR;

architecture rtl of f_OR is
begin
     y <= a or b;
end architecture rtl;

library ieee;
    use ieee.std_logic_1164.all;

entity main is
    port (
        a : in std_logic;
        b : in std_logic;
        c : in std_logic;
        y : out std_logic
    );
begin
end entity main;

architecture struct of main is
    signal y_and : std_logic;
begin
    u_f_add : entity work.f_AND
        port map (
            a => a,
            b => b,
            y => y_and
        );
    u_f_or : entity work.f_OR
        port map (
            a => y_and,
            b => c,
            y => y
        );
end architecture struct;

