use indoc::indoc;

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

pub fn generate_prelude() -> String {
    let mut prelude = String::new();
    prelude.push_str(&indoc!(
        "
        library ieee;
        use ieee.std_logic_1164.all;
        use ieee.numeric_std.all;

        entity SchemaParser is
        generic (
            EPC                   : natural := 8;
            INT_WIDTH             : natural := 16;
            INT_P_PIPELINE_STAGES : natural := 2;
            END_REQ_EN            : boolean := false
        );
        port (
            clk                   : in  std_logic;
            reset                 : in  std_logic;

            in_valid              : in  std_logic;
            in_ready              : out std_logic;
            in_data               : in  std_logic_vector(8*EPC-1 downto 0);
            in_last               : in  std_logic_vector(2*EPC-1 downto 0);
            in_stai               : in  std_logic_vector(log2ceil(EPC)-1 downto 0) := (others => '0');
            in_endi               : in  std_logic_vector(log2ceil(EPC)-1 downto 0) := (others => '1');
            in_strb               : in  std_logic_vector(EPC-1 downto 0);

            end_req               : in  std_logic := '0';
            end_ack               : out std_logic;

            out_valid             : out std_logic;
            out_ready             : in  std_logic;
            out_data              : out std_logic_vector(INT_WIDTH-1 downto 0);
            out_strb              : out std_logic;
            out_last              : out std_logic_vector(2 downto 0)

        );
        end entity;

        architecture arch of SchemaParser is
        begin
        ")
    );

    prelude
}

pub fn generate_postlude() -> String {
    let mut postlude = String::new();
    postlude.push_str("end arch;");

    postlude
}