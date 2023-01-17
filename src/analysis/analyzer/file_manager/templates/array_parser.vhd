library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.std_logic_misc.or_reduce;

library work;
use work.${project_name}.all;
use work.UtilInt_pkg.all;

entity ${namespace}_0_${comp_name}_com is
  generic (
    EPC : positive := 4;
    OUTER_NESTING_LEVEL : natural := 2;
    INNER_NESTING_LEVEL : natural := 0;
    ELEMENT_COUNTER_BW : natural := 4
  );
  port (
    clk : in std_logic;
    rst : in std_logic;
    input_valid : in std_logic;
    input_ready : out std_logic;
    input_data : in std_logic_vector(EPC*${bit_width}-1 downto 0);
    input_last : in std_logic_vector(((OUTER_NESTING_LEVEL + 1) * EPC) - 1 downto 0);
    input_stai : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    input_endi : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    input_strb : in std_logic_vector(EPC-1 downto 0);
    output_valid : out std_logic;
    output_ready : in std_logic;
    output_data : out std_logic_vector(EPC*${bit_width}-1 downto 0);
    output_last : out std_logic_vector(((OUTER_NESTING_LEVEL + 2) * EPC) - 1 downto 0);
    output_stai : out std_logic_vector(log2ceil(EPC)-1 downto 0);
    output_endi : out std_logic_vector(log2ceil(EPC)-1 downto 0);
    output_strb : out std_logic_vector(EPC-1 downto 0)
  );
end ${namespace}_0_${comp_name}_com;

architecture behav of ${namespace}_0_${comp_name}_com is
begin
  clk_proc: process (clk) is
    constant IDXW : natural := log2ceil(EPC);

    -- Input holding register.
    type in_type is record
      data  : std_logic_vector(7 downto 0);
      last  : std_logic_vector(OUTER_NESTING_LEVEL-1 downto 0);
      strb  : std_logic;
    end record;

    type in_array is array (natural range <>) of in_type;
    variable id : in_array(0 to EPC-1);
    variable iv : std_logic := '0';
    variable ir : std_logic := '0';

    -- Output holding register.
    type out_type is record
      data  : std_logic_vector(7 downto 0);
      last  : std_logic_vector(OUTER_NESTING_LEVEL+1 downto 0);
      strb  : std_logic;
    end record;

    type out_array is array (natural range <>) of out_type;
    variable od : out_array(0 to EPC-1);
    variable ov : std_logic := '0';

    variable stai    : unsigned(log2ceil(EPC)-1 downto 0);
    variable endi    : unsigned(log2ceil(EPC)-1 downto 0);
    variable idx_int : unsigned(log2ceil(EPC)-1 downto 0);

    -- Enumeration type for our state machine.
    type state_t is (STATE_IDLE,
                     STATE_ARRAY);

    -- State variable
    variable state : state_t;

    variable nesting_level_th : std_logic_vector(INNER_NESTING_LEVEL downto 0) := (others => '0');
    variable nesting_inner    : std_logic_vector(INNER_NESTING_LEVEL downto 1) := (others => '0');

    variable is_top_array     : std_logic;

  begin
    if rising_edge(clk) then

      -- Latch input holding register if we said we would.
      if to_x01(ir) = '1' then
        iv := input_valid;
        stai      := to_unsigned(0, stai'length);
        endi      := to_unsigned(EPC-1, endi'length);
        for idx in 0 to EPC-1 loop
          id(idx).data := input_data(8*idx+7 downto 8*idx);
          id(idx).last := input_last((OUTER_NESTING_LEVEL+1)*(idx+1)-1 downto (OUTER_NESTING_LEVEL+1)*idx+1);
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
        for idx in 0 to EPC-1 loop

          -- Default behavior.
          od(idx).data       := id(idx).data;
          od(idx).last(OUTER_NESTING_LEVEL+1 downto 0)   := id(idx).last & "00";
          od(idx).strb       := '0';
          
          -- Element-wise processing only when the lane is valid.
          if to_x01(id(idx).strb) = '1' then


            -- Keep track of nesting.
            case id(idx).data is
              when X"7B" => -- '{'
                nesting_level_th := nesting_level_th(nesting_level_th'high-1 downto 0) & '1';
              when X"5B" => -- '['
                nesting_level_th := nesting_level_th(nesting_level_th'high-1 downto 0) & '1';
              when X"7D" => -- '}'
                nesting_level_th := '0' &nesting_level_th(nesting_level_th'high downto 1);
              when X"5D" => -- ']'
                nesting_level_th := '0' &nesting_level_th(nesting_level_th'high downto 1);
              when others =>
                nesting_level_th := nesting_level_th;
            end case;

            nesting_inner := nesting_level_th(nesting_level_th'high downto 1);
            is_top_array  := nesting_level_th(0);

            case state is
              when STATE_IDLE =>
                case id(idx).data is
                  when X"5B" => -- '['
                    state := STATE_ARRAY;
                  when others =>
                    state := STATE_IDLE;
                end case;

              when STATE_ARRAY =>
                od(idx).strb := '1';
                ov           := '1';
                case id(idx).data is
                  when X"5D" => -- ']'
                    if or_reduce(nesting_inner) = '0' then
                      -- Keep processing values if we are still in an inner array.
                      if is_top_array = '1' then
                        state := STATE_ARRAY;
                      else
                        state := STATE_IDLE;
                        od(idx).last(0) := '1';
                        od(idx).last(1) := '1';
        
                        od(idx).strb   := '0';
                      end if;
                    end if;
                  when X"2C" => -- ','
                    if or_reduce(nesting_inner) = '0' then
                      state := STATE_ARRAY;
                      od(idx).last(0) := '1';
                      od(idx).strb   := '0';
                    end if;
                  when others =>
                    state := STATE_ARRAY;
                end case;
            end case;
          end if;
          -- Clear state upon any last, to prevent broken elements from messing
          -- up everything.
          if or_reduce(id(idx).last) /= '0' then
            state := STATE_IDLE;
          end if;
        end loop;

        for idx in 0 to EPC-1 loop
          if or_reduce(od(idx).last) = '1' then
            ov := '1';
          end if;
        end loop;
        iv := '0';
      end if;

      -- Handle reset.
      if to_x01(rst) /= '0' then
        iv    := '0';
        ov    := '0';
        state := STATE_IDLE;
      end if;

      -- Forward output holding register.
      output_valid <= to_x01(ov);
      ir := not iv and not rst;
      input_ready <= ir and not rst;
      for idx in 0 to EPC-1 loop
        output_data(8*idx+7 downto 8*idx) <= od(idx).data;
        output_last((OUTER_NESTING_LEVEL+2)*(idx+1)-1 downto (OUTER_NESTING_LEVEL+2)*idx) <= od(idx).last;
        output_stai <= std_logic_vector(stai);
        output_endi <= std_logic_vector(endi);
        output_strb(idx) <= od(idx).strb;
      end loop;
    end if;
  end process;
end behav;