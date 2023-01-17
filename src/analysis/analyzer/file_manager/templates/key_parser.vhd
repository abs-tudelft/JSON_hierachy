library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.std_logic_misc.or_reduce;

library work;
use work.${project_name}.all;
use work.UtilInt_pkg.all;
use work.Stream_pkg.all;

entity ${namespace}_0_${comp_name}_com is
  generic (
    EPC : positive := 4;
    OUTER_NESTING_LEVEL : positive := 2;
    DLY_COMP_BUFF_DEPTH  : integer := 5
  );
  port (
    clk : in std_logic;
    rst : in std_logic;
    input_valid : in std_logic;
    input_ready : out std_logic;
    input_data : in std_logic_vector(EPC*${bit_width}+EPC-1 downto 0);
    input_last : in std_logic_vector(((OUTER_NESTING_LEVEL + 1) * EPC) - 1 downto 0);
    input_stai : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    input_endi : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    input_strb : in std_logic_vector(EPC-1 downto 0);
    matcher_str_valid : out std_logic;
    matcher_str_ready : in std_logic;
    matcher_str_data : out std_logic_vector(EPC*${bit_width}-1 downto 0);
    matcher_str_last : out std_logic_vector(EPC-1 downto 0);
    matcher_str_stai : out std_logic_vector(log2ceil(EPC)-1 downto 0);
    matcher_str_endi : out std_logic_vector(log2ceil(EPC)-1 downto 0);
    matcher_str_strb : out std_logic_vector(EPC-1 downto 0);
    matcher_match_valid : in std_logic;
    matcher_match_ready : out std_logic;
    matcher_match_data : in std_logic_vector(EPC-1 downto 0);
    matcher_match_last : in std_logic_vector(EPC-1 downto 0);
    matcher_match_stai : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    matcher_match_endi : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    matcher_match_strb : in std_logic_vector(EPC-1 downto 0);
    output_valid : out std_logic;
    output_ready : in std_logic;
    output_data : out std_logic_vector(EPC*${bit_width}-1 downto 0);
    output_last : out std_logic_vector(((OUTER_NESTING_LEVEL + 1) * EPC) - 1 downto 0);
    output_stai : out std_logic_vector(log2ceil(EPC)-1 downto 0);
    output_endi : out std_logic_vector(log2ceil(EPC)-1 downto 0);
    output_strb : out std_logic_vector(EPC-1 downto 0)
  );
end ${namespace}_0_${comp_name}_com;

architecture behav of ${namespace}_0_${comp_name}_com is
  constant IN_VEC_WIDTH         : integer := 8*EPC + EPC;
  constant IN_DATA_STAI         : integer := 0;
  constant IN_DATA_ENDI         : integer := EPC*8-1;
  CONSTANT IN_TAG_STAI          : integer := EPC*8;
  CONSTANT IN_TAG_ENDI          : integer := EPC*8+EPC-1;
  
  -- Index constants for packing input into a single vector.
  constant BUFF_WIDTH            : integer := EPC*(2 + 8 + OUTER_NESTING_LEVEL+1);
  constant BUFF_DATA_STAI        : integer := 0;
  constant BUFF_DATA_ENDI        : integer := EPC*8-1;
  constant BUFF_TAG_STAI         : integer := EPC*8;
  constant BUFF_TAG_ENDI         : integer := EPC*8 + EPC -1;
  constant BUFF_STRB_STAI        : integer := EPC*8 + EPC;
  constant BUFF_STRB_ENDI        : integer := EPC*8 + 2*EPC-1;
  constant BUFF_LAST_STAI        : integer := EPC*8 + 2*EPC;
  constant BUFF_LAST_ENDI        : integer := EPC*8 + 2*EPC + (OUTER_NESTING_LEVEL+1)*EPC-1;


  signal matcher_slice_in        : std_logic_vector(2*EPC-1 downto 0);
  signal matcher_slice_out       : std_logic_vector(2*EPC-1 downto 0);

  signal matcher_match_valid_s   : std_logic;
  signal matcher_match_ready_s   : std_logic;
  signal matcher_match_strb_s    : std_logic_vector(EPC-1 downto 0);
  signal matcher_match_s         : std_logic_vector(EPC-1 downto 0);

  signal buff_in_valid           : std_logic;
  signal buff_in_valid_t         : std_logic;
  signal buff_in_ready           : std_logic;
  signal buff_in_data            : std_logic_vector(BUFF_WIDTH-1 downto 0);

  signal buff_out_valid          : std_logic;
  signal buff_out_ready          : std_logic;
  signal buff_out_data           : std_logic_vector(BUFF_WIDTH-1 downto 0);


  begin

    dly_comp_buff: StreamBuffer
      generic map (
        DATA_WIDTH              => BUFF_WIDTH,
        MIN_DEPTH               => DLY_COMP_BUFF_DEPTH
      )
      port map (
        clk                     => clk,
        reset                   => rst,
        in_valid                => buff_in_valid_t,
        in_ready                => buff_in_ready,
        in_data                 => buff_in_data,
        out_valid               => buff_out_valid,
        out_ready               => buff_out_ready,
        out_data                => buff_out_data
      );

      matcher_slice_in(EPC-1 downto 0)     <= matcher_match_data;
      matcher_slice_in(2*EPC-1 downto EPC) <= matcher_match_strb;

      matcher_match_s      <= matcher_slice_out(EPC-1 downto 0);
      matcher_match_strb_s <= matcher_slice_out(2*EPC-1 downto EPC);
      
      matcher_slice: StreamSlice
      generic map (
        DATA_WIDTH                  => EPC*2
      )
      port map (
        clk                     => clk,
        reset                   => rst,
        in_valid                => matcher_match_valid,
        in_ready                => matcher_match_ready,
        in_data                 => matcher_slice_in,
        out_valid               => matcher_match_valid_s,
        out_ready               => matcher_match_ready_s,
        out_data                => matcher_slice_out
      );

    in_sync: StreamSync
      generic map (
        NUM_INPUTS              => 1,
        NUM_OUTPUTS             => 2
      )
      port map (
        clk                     => clk,
        reset                   => rst,
        in_valid(0)             => input_valid,
        in_ready(0)             => input_ready,
        out_valid(0)            => buff_in_valid,
        out_valid(1)            => matcher_str_valid,
        out_ready(0)            => buff_in_ready,
        out_ready(1)            => matcher_str_ready
      );

    input_interfacing: process (input_data, input_last, input_strb, buff_in_valid, input_stai, input_endi) is
      variable strb         :  std_logic_vector(EPC-1 downto 0);
      variable last         :  std_logic_vector(EPC-1 downto 0);
      variable in_data_f    :  std_logic_vector(EPC*8-1 downto 0);
      variable in_tag_f     :  std_logic_vector(EPC-1 downto 0);
    begin
      for idx in 0 to EPC-1 loop
        if idx < unsigned(input_stai) then
          strb(idx) := '0';
        elsif idx > unsigned(input_endi) then
          strb(idx) := '0';
        else
          strb(idx) := input_strb(idx);
        end if;
        last(idx) := input_last((OUTER_NESTING_LEVEL+1)*idx);
      end loop;

      in_data_f := input_data(IN_DATA_ENDI downto IN_DATA_STAI);
      in_tag_f  := input_data(IN_TAG_ENDI downto IN_TAG_STAI);

      -- Pack buffer data.
      buff_in_data(BUFF_DATA_ENDI downto BUFF_DATA_STAI)    <= in_data_f;
      buff_in_data(BUFF_TAG_ENDI downto BUFF_TAG_STAI)      <= in_tag_f;
      buff_in_data(BUFF_STRB_ENDI downto BUFF_STRB_STAI)    <= strb;
      buff_in_data(BUFF_LAST_ENDI downto BUFF_LAST_STAI)    <= input_last;

      matcher_str_data <= to_stdlogicvector(to_bitvector(input_data(IN_DATA_ENDI downto IN_DATA_STAI))); -- Metavalue wanings fix. VERY DIRTY!!!
      matcher_str_strb <= strb and (not in_tag_f);
      matcher_str_last <= last and (not in_tag_f);
      matcher_str_endi <= (others => '1');

      buff_in_valid_t <= buff_in_valid and (or_reduce(in_tag_f) or or_reduce(input_last));
    end process;

    filter_proc: process (clk) is
      constant IDXW : natural := log2ceil(EPC);
  
      -- Input holding register.
      type in_type is record
        data       : std_logic_vector(7 downto 0);
        last       : std_logic_vector(OUTER_NESTING_LEVEL downto 0);
        match      : std_logic;
        match_strb : std_logic;
        tag        : std_logic;
        strb       : std_logic;
      end record;
    
      type in_array is array (natural range <>) of in_type;
      variable id : in_array(0 to EPC-1);
      variable bv : std_logic := '0';
      variable mv : std_logic := '0';
      variable br : std_logic := '0';
      variable mr : std_logic := '0';
      
  
      -- Output holding register.
      type out_type is record
        data  : std_logic_vector(7 downto 0);
        last  : std_logic_vector(OUTER_NESTING_LEVEL downto 0);
        strb  : std_logic;
      end record;
  
      type out_array is array (natural range <>) of out_type;
      variable od : out_array(0 to EPC-1);
      variable ov : std_logic := '0';

      variable outer_last : std_logic;
      variable match_last : std_logic;
  
      -- Enumeration type for our state machine.
      type state_t is (STATE_IDLE,
                       STATE_MATCH,
                       STATE_DROP);
  
      -- State variable
      variable state : state_t;
  
    begin
    
      if rising_edge(clk) then
  
        -- Latch buffer input holding register.
        if to_x01(br) = '1' then
          bv := buff_out_valid;

          for idx in 0 to EPC-1 loop
            id(idx).data  := buff_out_data(BUFF_DATA_STAI+idx*8+7 downto BUFF_DATA_STAI+idx*8);
            id(idx).tag   := buff_out_data(BUFF_TAG_STAI+idx);
            id(idx).strb  := buff_out_data(BUFF_STRB_STAI+idx);
            id(idx).last  := buff_out_data(BUFF_LAST_STAI+(OUTER_NESTING_LEVEL+1)*idx+OUTER_NESTING_LEVEL downto BUFF_LAST_STAI+(OUTER_NESTING_LEVEL+1)*idx);
          end loop;
        end if;

        if to_x01(mr) = '1' then
          mv := matcher_match_valid_s;
          for idx in 0 to EPC-1 loop
            id(idx).match      := matcher_match_s(idx);
            id(idx).match_strb := matcher_match_strb_s(idx);
          end loop;
        end if;
  
        -- Clear output holding register if transfer was accepted.
        if to_x01(output_ready) = '1' then
          if ov = '1' then
            match_last := '0';
            for idx in 0 to EPC-1 loop
              od(idx).last(0) := '0';
              od(idx).strb    := '0';
            end loop;
          end if;
          ov         := '0';
        end if;
        
        -- Do processing when both registers are ready.
        if to_x01(bv) = '1' and to_x01(ov) = '0' then
          outer_last := '0';
          bv         := '0';
          for idx in 0 to EPC-1 loop
  
            -- Default behavior.
            od(idx).data                                  := id(idx).data;
            od(idx).last(OUTER_NESTING_LEVEL downto 1)    := id(idx).last(OUTER_NESTING_LEVEL downto 1);

            -- Pass transfers that close out outer dimensions. 
            if or_reduce(id(idx).last(OUTER_NESTING_LEVEL downto 1)) = '1' then
              outer_last := '1';
            end if;

            case state is
              when STATE_IDLE =>
                if outer_last = '1' and bv = '0' then
                  ov := '1';
                end if;
                -- If we get an innermost last in a key, that's gonna trigger the matcher, so keep it.
                if id(idx).last(0) = '1' and id(idx).tag = '0' then
                  bv := '1';
                  ov := '0';
                  if to_x01(mv) = '1' then
                    mv := '0';
                    ov := '0';
                    bv := '0';
                    if outer_last = '1' or match_last = '1'then
                      ov         := '1';
                    end if;
                    if to_x01(id(idx).match_strb) = '1' then
                      if to_x01(id(idx).match) = '1' then
                        state := STATE_MATCH;
                      else
                        state := STATE_DROP;
                      end if;
                    end if;
                  end if;
                end if;
              when STATE_MATCH =>
                ov := '1';
                od(idx).strb := id(idx).strb;
                if id(idx).last(0) = '1' and id(idx).tag = '1' then
                  state := STATE_IDLE;
                  od(idx).strb := '0';
                  od(idx).last(0) := '1';
                  match_last := '1';
                end if;
              when STATE_DROP =>
                if outer_last = '1' then
                  ov := '1';
                end if;
                if id(idx).last(0) = '1' and id(idx).tag = '1' then
                  state := STATE_IDLE;
                end if;
            end case;
          end loop;
        end if;
  
        -- Handle reset.
        if to_x01(rst) /= '0' then
          bv         := '0';
          mv         := '0';
          ov         := '0';
          state      := STATE_IDLE;
          match_last := '0';
          for idx in 0 to EPC-1 loop
            od(idx).last(0) := '0';
            od(idx).strb    := '0';
          end loop;
        end if;
  
        -- Forward output holding register.
        output_valid <= to_x01(ov);
        br := not bv and not rst;
        mr := not mv and not rst;
        buff_out_ready <= br;
        matcher_match_ready_s <= mr;
        for idx in 0 to EPC-1 loop
          output_data(8*idx+7 downto 8*idx) <= od(idx).data;
          output_last((OUTER_NESTING_LEVEL+1)*(idx+1)-1 downto (OUTER_NESTING_LEVEL+1)*idx) <= od(idx).last;
          output_stai <= (others => '0');
          output_endi <= (others => '1');
          output_strb(idx) <= od(idx).strb;
        end loop;
      end if;
    end process;
end behav;