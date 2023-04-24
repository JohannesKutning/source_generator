--
-- Automatically generated on 2023-02-02 19:33:57.
-- Do _NOT_ edit!
--
library ieee;
    use ieee.std_logic_1164.all;

entity host is
    generic (
        ADDR_WIDTH : positive;
        DATA_WIDTH : positive;
        BYTE_COUNT : positive
    );
    port (
        address : out std_logic_vector( ADDR_WIDTH - 1 downto 0 );
        byteenable : out std_logic_vector( BYTE_COUNT - 1 downto 0 );
        read : out std_logic;
        read_data : in std_logic_vector( DATA_WIDTH - 1 downto 0 );
        response : in std_logic_vector( 1 downto 0 );
        write : out std_logic;
        write_data : out std_logic_vector( DATA_WIDTH - 1 downto 0 )
    );
begin
end entity host;

library ieee;
    use ieee.std_logic_1164.all;

entity agent is
    generic (
        ADDR_WIDTH : positive;
        DATA_WIDTH : positive;
        BYTE_COUNT : positive
    );
    port (
        address : in std_logic_vector( ADDR_WIDTH - 1 downto 0 );
        byteenable : in std_logic_vector( BYTE_COUNT - 1 downto 0 );
        read : in std_logic;
        read_data : out std_logic_vector( DATA_WIDTH - 1 downto 0 );
        response : out std_logic_vector( 1 downto 0 );
        write : in std_logic;
        write_data : in std_logic_vector( DATA_WIDTH - 1 downto 0 )
    );
begin
end entity agent;

library ieee;
    use ieee.std_logic_1164.all;

entity main is
    generic (
        ADDR_WIDTH : positive;
        DATA_WIDTH : positive;
        BYTE_COUNT : positive
    );
begin
end entity main;

architecture struct of main is
    signal host_to_agent_address : std_logic_vector( ADDR_WIDTH - 1 downto 0 );
    signal host_to_agent_byteenable : std_logic_vector( BYTE_COUNT - 1 downto 0 );
    signal host_to_agent_read : std_logic;
    signal agent_to_host_read_data : std_logic_vector( DATA_WIDTH - 1 downto 0 );
    signal agent_to_host_response : std_logic_vector( 1 downto 0 );
    signal host_to_agent_write : std_logic;
    signal host_to_agent_write_data : std_logic_vector( DATA_WIDTH - 1 downto 0 );
begin
    host : entity work.host
        generic map (
            ADDR_WIDTH => ADDR_WIDTH,
            DATA_WIDTH => DATA_WIDTH,
            BYTE_COUNT => BYTE_COUNT
        )
        port map (
            address => host_to_agent_address,
            byteenable => host_to_agent_byteenable,
            read => host_to_agent_read,
            read_data => agent_to_host_read_data,
            response => agent_to_host_response,
            write => host_to_agent_write,
            write_data => host_to_agent_write_data
        );
    agent : entity work.agent
        generic map (
            ADDR_WIDTH => ADDR_WIDTH,
            DATA_WIDTH => DATA_WIDTH,
            BYTE_COUNT => BYTE_COUNT
        )
        port map (
            address => host_to_agent_address,
            byteenable => host_to_agent_byteenable,
            read => host_to_agent_read,
            read_data => agent_to_host_read_data,
            response => agent_to_host_response,
            write => host_to_agent_write,
            write_data => host_to_agent_write_data
        );
end architecture struct;

