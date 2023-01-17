library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.std_logic_misc.or_reduce;

library work;
use work.${project_name}.all;
use work.UtilInt_pkg.all;

entity ${namespace}_0_${comp_name}_com is
  generic (
      EPC                   : positive := 1;
      NESTING_LEVEL         : positive := 1
      );
  port (
      clk                   : in  std_logic;
      rst                   : in  std_logic;

      -- Stream(
      --     Bits(8),
      --     t=EPC,
      --     d=NESTING_LEVEL+1,
      --     c=8
      -- )
      input_valid              : in  std_logic;
      input_ready              : out std_logic;
      input_data               : in  std_logic_vector(8*EPC-1 downto 0);
      input_last               : in  std_logic_vector((NESTING_LEVEL+1)*EPC-1 downto 0) := (others => '0');
      input_stai               : in  std_logic_vector(log2ceil(EPC)-1 downto 0) := (others => '0');
      input_endi               : in  std_logic_vector(log2ceil(EPC)-1 downto 0) := (others => '1');
      input_strb               : in  std_logic_vector(EPC-1 downto 0) := (others => '1');

      -- Stream(
      --     Bits(1),
      --     d=NESTING_LEVEL,
      --     c=8
      -- )
      output_valid             : out std_logic;
      output_ready             : in  std_logic;
      output_data              : out std_logic;
      output_strb              : out std_logic;
      output_last              : out std_logic_vector(NESTING_LEVEL-1 downto 0)
  );
end ${namespace}_0_${comp_name}_com;

architecture behav of ${namespace}_0_${comp_name}_com is
    begin
      clk_proc: process (clk) is
        constant IDXW : natural := log2ceil(EPC);
    
        -- Input holding register.
        type in_type is record
          data  : std_logic_vector(7 downto 0);
          last  : std_logic_vector(NESTING_LEVEL downto 0);
          empty : std_logic;
          strb  : std_logic;
        end record;
    
        type in_array is array (natural range <>) of in_type;
        variable id : in_array(0 to EPC-1);
        variable iv : std_logic := '0';
        variable ir : std_logic := '0';
    
        variable ov : std_logic := '0';
        variable oe : std_logic := '1';
        variable ol : std_logic_vector(NESTING_LEVEL-1 downto 0) := (others => '0');

        variable val : std_logic;
    
      begin
        if rising_edge(clk) then
    
          -- Latch input holding register if we said we would.
          if to_x01(ir) = '1' then
            iv := input_valid;
            for idx in 0 to EPC-1 loop
              id(idx).data := input_data(8*idx+7 downto 8*idx);
              id(idx).last := input_last((NESTING_LEVEL+1)*(idx+1)-1 downto (NESTING_LEVEL+1)*idx);
              if idx < unsigned(input_stai) then
                id(idx).strb := '0';
              elsif idx > unsigned(input_endi) then
                id(idx).strb := '0';
              else
                id(idx).strb := input_strb(idx);
              end if;
            end loop;
          end if;
    
          -- Clear output holding register if transfer was accepted.
          if to_x01(output_ready) = '1' then
            ov := '0';
          end if;

          -- Do processing when both registers are ready.
          if to_x01(iv) = '1' and to_x01(ov) /= '1' then
            oe := '1';
            ol := (others => '0');
            for idx in 0 to EPC-1 loop
              ol := ol or id(idx).last(NESTING_LEVEL downto 1);
              id(idx).last(NESTING_LEVEL downto 1) := (others => '0');
              if to_x01(id(idx).strb) = '1' and to_x01(ov) /= '1' then
                case id(idx).data is
                  when X"66" => -- 'f'
                      ov := '1';
                      oe := '0';
                      val:= '0';
                  when X"46" => -- 'F'
                      ov := '1';
                      oe := '0';
                      val:= '0';
                  when X"74" => -- 't'
                      ov := '1';
                      oe := '0';    
                      val:= '1';
                  when X"54" => -- 'T'
                      ov := '1';
                      oe := '0';
                      val:= '1';
                  when others =>
                      ov := '0';
                end case;
              end if;
              id(idx).strb := '0';
            end loop;
            iv := '0';
            for idx in id'range loop
              if id(idx).strb = '1' or or_reduce(id(idx).last(NESTING_LEVEL downto 1)) = '1' then
                iv := '1';
              end if;
            end loop;
            if or_reduce(ol) = '1' and iv = '0' then
              ov := '1';
            end if;
          end if;
    
          -- Handle rst.
          if to_x01(rst) /= '0' then
            iv    := '0';
            ov    := '0';
          end if;
    
          -- Forward output holding register.
          ir := not iv and not rst;
          input_ready <= ir and not rst;
          output_valid <= to_x01(ov);
          output_data <= val;
          output_last <= ol;
          output_strb <= not oe;
        end if;
      end process;
    end architecture;