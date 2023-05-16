--
-- Automatically generated on 2023-04-27 00:19:19.
-- Do _NOT_ edit!
--
library ieee;
    use ieee.std_logic_1164.all;

entity sub is
    generic (
        ADDR_WIDTH : positive;
        DATA_WIDTH : positive;
        BYTE_COUNT : positive
    );
    port (
        host_address : out std_logic_vector( ADDR_WIDTH - 1 downto 0 );
        host_byteenable : out std_logic_vector( BYTE_COUNT - 1 downto 0 );
        host_read : out std_logic;
        host_read_data : in std_logic_vector( DATA_WIDTH - 1 downto 0 );
        host_response : in std_logic_vector( 1 downto 0 );
        host_write : out std_logic;
        host_write_data : out std_logic_vector( DATA_WIDTH - 1 downto 0 );
        agent_address : in std_logic_vector( ADDR_WIDTH - 1 downto 0 );
        agent_byteenable : in std_logic_vector( BYTE_COUNT - 1 downto 0 );
        agent_read : in std_logic;
        agent_read_data : out std_logic_vector( DATA_WIDTH - 1 downto 0 );
        agent_response : out std_logic_vector( 1 downto 0 );
        agent_write : in std_logic;
        agent_write_data : in std_logic_vector( DATA_WIDTH - 1 downto 0 )
    );
begin
end entity sub;

library ieee;
    use ieee.std_logic_1164.all;

entity main is
    generic (
        ADDR_WIDTH : positive;
        DATA_WIDTH : positive;
        BYTE_COUNT : positive
    );
    port (
        host_address : out std_logic_vector( ADDR_WIDTH - 1 downto 0 );
        host_byteenable : out std_logic_vector( BYTE_COUNT - 1 downto 0 );
        host_read : out std_logic;
        host_read_data : in std_logic_vector( DATA_WIDTH - 1 downto 0 );
        host_response : in std_logic_vector( 1 downto 0 );
        host_write : out std_logic;
        host_write_data : out std_logic_vector( DATA_WIDTH - 1 downto 0 );
        agent_address : in std_logic_vector( ADDR_WIDTH - 1 downto 0 );
        agent_byteenable : in std_logic_vector( BYTE_COUNT - 1 downto 0 );
        agent_read : in std_logic;
        agent_read_data : out std_logic_vector( DATA_WIDTH - 1 downto 0 );
        agent_response : out std_logic_vector( 1 downto 0 );
        agent_write : in std_logic;
        agent_write_data : in std_logic_vector( DATA_WIDTH - 1 downto 0 )
    );
begin
end entity main;

architecture struct of main is
    signal left_to_right_host_address : std_logic_vector( ADDR_WIDTH - 1 downto 0 );
    signal left_to_right_host_byteenable : std_logic_vector( BYTE_COUNT - 1 downto 0 );
    signal left_to_right_host_read : std_logic;
    signal right_to_left_host_read_data : std_logic_vector( DATA_WIDTH - 1 downto 0 );
    signal right_to_left_host_response : std_logic_vector( 1 downto 0 );
    signal left_to_right_host_write : std_logic;
    signal left_to_right_host_write_data : std_logic_vector( DATA_WIDTH - 1 downto 0 );
begin
    left : entity work.sub
        generic map (
            ADDR_WIDTH => ADDR_WIDTH,
            DATA_WIDTH => DATA_WIDTH,
            BYTE_COUNT => BYTE_COUNT
        )
        port map (
            host_address => left_to_right_host_address,
            host_byteenable => left_to_right_host_byteenable,
            host_read => left_to_right_host_read,
            host_read_data => right_to_left_host_read_data,
            host_response => right_to_left_host_response,
            host_write => left_to_right_host_write,
            host_write_data => left_to_right_host_write_data,
            agent_address => agent_address,
            agent_byteenable => agent_byteenable,
            agent_read => agent_read,
            agent_read_data => agent_read_data,
            agent_response => agent_response,
            agent_write => agent_write,
            agent_write_data => agent_write_data
        );
    right : entity work.sub
        generic map (
            ADDR_WIDTH => ADDR_WIDTH,
            DATA_WIDTH => DATA_WIDTH,
            BYTE_COUNT => BYTE_COUNT
        )
        port map (
            host_address => host_address,
            host_byteenable => host_byteenable,
            host_read => host_read,
            host_read_data => host_read_data,
            host_response => host_response,
            host_write => host_write,
            host_write_data => host_write_data,
            agent_address => left_to_right_host_address,
            agent_byteenable => left_to_right_host_byteenable,
            agent_read => left_to_right_host_read,
            agent_read_data => right_to_left_host_read_data,
            agent_response => right_to_left_host_response,
            agent_write => left_to_right_host_write,
            agent_write_data => left_to_right_host_write_data
        );
end architecture struct;

