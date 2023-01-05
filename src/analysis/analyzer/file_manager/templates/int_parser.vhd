library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use ieee.std_logic_misc.all;

library work;
use work.${project_name}.all;
use work.UtilInt_pkg.all;

entity ${namespace}_0_${comp_name}_com is
  generic (
    EPC : positive := 4;
    DIM : positive := 4;
    NESTING_LEVEL : positive := 3;
    BITWIDTH : positive := 64;
    PIPELINE_STAGES : natural := 1
  );
  port (
    clk : in std_logic;
    rst : in std_logic;
    input_valid : in std_logic;
    input_ready : out std_logic;
    input_data : in std_logic_vector(EPC*${bit_width} downto 0);
    input_last : in std_logic_vector((DIM * EPC) - 1 downto 0);
    input_stai : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    input_endi : in std_logic_vector(log2ceil(EPC)-1 downto 0);
    input_strb : in std_logic_vector(EPC-1 downto 0);
    output_valid : out std_logic;
    output_ready : in std_logic;
    output_data : out std_logic_vector(${int_width}-1 downto 0);
    output_last : out std_logic_vector((DIM - 1) - 1 downto 0);
    output_strb : out std_logic
  );
end ${namespace}_0_${comp_name}_com;

architecture behav of ${namespace}_0_${comp_name}_com is
-- Input holding register.
type in_type is record
  data  : std_logic_vector(7 downto 0);
  last  : std_logic_vector(NESTING_LEVEL downto 0);
  strb  : std_logic;
end record;

type dd_stage_t is record
  bcd   : std_logic_vector(BITWIDTH+(BITWIDTH-4)/3-1 downto 0);
  bin   : std_logic_vector(BITWIDTH-1 downto 0);
  ready : std_logic;
  valid : std_logic;
  empty : std_logic;
  last  : std_logic_vector(NESTING_LEVEL-1 downto 0);
end record;

constant BCD_WIDTH    : integer := BITWIDTH+(BITWIDTH-4)/3;

constant dd_stage_t_init : dd_stage_t := (  bcd => (others => '0'),
                                            bin => (others => '0'),
                                            ready => '0',
                                            valid => '0',
                                            empty => '1',
                                            last => (others => '0'));

signal dd_in_s  : dd_stage_t := dd_stage_t_init;
signal dd_out_s : dd_stage_t := dd_stage_t_init;


type pipeline_reg_array_t is array (natural range <>) of dd_stage_t;
signal stage_in_array : pipeline_reg_array_t(0 to PIPELINE_STAGES-1) := (others=>(dd_stage_t_init));
signal stage_out_array : pipeline_reg_array_t(0 to PIPELINE_STAGES-1) := (others=>(dd_stage_t_init));

type stage_data_t is array (0 to PIPELINE_STAGES) of dd_stage_t;

signal dd_stage_data_in : stage_data_t := (others => dd_stage_t_init);
signal dd_stage_data_out : stage_data_t := (others => dd_stage_t_init);

signal dd_ready : std_logic;


procedure dd_stage (
    signal    i         : in  dd_stage_t;
    signal    o         : out dd_stage_t;
    constant  BW        : in natural;
    constant  STEPS     : in natural
  ) is
    variable bcd_shr : std_logic_vector(BW+(BW-4)/3-1 downto 0) := (others => '0');
    variable bin_shr : std_logic_vector(BW-1 downto 0) := (others => '0');
begin
  -- Use the double-dabble alogorithm to convert BCD to binary.
  bcd_shr := i.bcd;
  bin_shr := i.bin;
  for j in 0 to STEPS-1 loop
    bin_shr := bcd_shr(0) & bin_shr(bin_shr'left downto 1);
    bcd_shr := '0' & bcd_shr(bcd_shr'high downto 1);
    for idx in 0 to (BW+(BW-4)/3)/4-1 loop
      if to_01(unsigned(bcd_shr(idx*4+3 downto idx*4))) >= 8 then
        bcd_shr(idx*4+3 downto idx*4) := std_logic_vector(unsigned(unsigned(bcd_shr(idx*4+3 downto idx*4)) - 3));
      end if;
    end loop;
  end loop;
  o.bcd   <= bcd_shr;
  o.bin   <= bin_shr;
  o.last  <= i.last;
  o.empty <= i.empty;
  o.valid <= i.valid;
end procedure;

begin
    in_stage: process (clk) is
      
      type in_array is array (natural range <>) of in_type;
      variable id   : in_array(0 to EPC-1);
      variable stai : unsigned(log2ceil(EPC)-1 downto 0);
      variable iv   : std_logic := '0';
      variable ir   : std_logic := '0';

      variable in_shr  : std_logic_vector(BITWIDTH+(BITWIDTH-4)/3-1 downto 0) := (others => '0');

      variable dd_in  : dd_stage_t := dd_stage_t_init;

  begin

    assert BITWIDTH mod PIPELINE_STAGES = 0 Report "BITWIDTH mod PIPELINE_STAGES needs to be 0 in IntParser!"
    severity Failure;

    if rising_edge(clk) then
      -- Latch input holding register if we said we would.
      if to_x01(ir) = '1' then
        iv := input_valid;
        if to_x01(iv) = '1'then
          for idx in 0 to EPC-1 loop
            id(idx).data := input_data(8*idx+7 downto 8*idx);
            id(idx).last := input_last((NESTING_LEVEL+1)*(idx+1)-1 downto (NESTING_LEVEL+1)*idx);
            stai := unsigned(input_stai);
            id(idx).strb := input_strb(idx);
            if idx < unsigned(input_stai) then
              id(idx).strb := '0';
            elsif idx > unsigned(input_endi) then
              id(idx).strb := '0';
            else
              id(idx).strb := input_strb(idx);
            end if;
          end loop;
        end if;
      end if;

      -- Clear output holding register if transfer was accepted.
      if to_x01(dd_ready) = '1' then
        if dd_in.valid = '1' then
          dd_in := dd_stage_t_init;
        end if;
        dd_in.valid := '0';
      end if;

      for idx in 0 to EPC-1 loop
        if to_x01(iv) = '1' and to_x01(dd_in.valid) = '0' then

          dd_in.last := dd_in.last or id(idx).last(NESTING_LEVEL downto 1);
          id(idx).last(NESTING_LEVEL downto 1) := (others => '0');

          if to_x01(id(idx).strb) = '1' or id(idx).last(0) /= '0' then

            if id(idx).data(7 downto 4) = X"3" then
              in_shr := in_shr(in_shr'high-4 downto 0) & id(idx).data(3 downto 0);
            end if;

            if id(idx).last(0) /= '0'  then
              id(idx).last(0) := '0';
              dd_in.bcd       := in_shr;
              in_shr          := (others => '0');
              dd_in.empty     := '0';
              dd_in.valid     := '1';
            end if;
          end if;
          id(idx).strb := '0';
        end if;
      end loop;

      if to_x01(iv) = '1'then
        iv := '0';
        for lane in id'range loop
          if id(lane).strb = '1' or or_reduce(id(lane).last(NESTING_LEVEL downto 1)) /= '0' then
            iv := '1';
          end if;
        end loop;
      end if;


      if or_reduce(dd_in.last) = '1' then
        dd_in.valid := '1';
      end if;

      -- Handle reset.
      if to_x01(rst) /= '0' then
        iv            := '0';
        dd_in         := dd_stage_t_init;
        in_shr        := (others => '0');
      end if;

      -- Assign input ready and forward data to the next stage.
      ir            := not iv;
      input_ready      <= ir;
      dd_in_s       <= dd_in;
      --bcd <=dd_in.bcd;

    end if;
  end process;

  pipeline_reg_proc: process (clk) is
    variable out_reg  : dd_stage_t := dd_stage_t_init;
    variable skid_reg : dd_stage_t := dd_stage_t_init;
    variable pr   : std_logic := '0';
    variable skid : std_logic := '0';
    begin

      if rising_edge(clk) then

        if to_x01(rst) /= '0' then
          pr := '0';
          skid := '0';
        end if;

        if output_ready = '1' then
          out_reg.valid := '0';
        end if;

        pr := not out_reg.valid;

        if pr = '0' then
          if skid = '0' then
            skid_reg := dd_in_s;
            skid := '1';
          end if;
        end if;

        if to_x01(rst) /= '0' then
          stage_in_array(0).valid   <= '0';
        elsif pr = '1' then
          if skid = '0' then
            stage_in_array(0) <= dd_in_s;
          else
            stage_in_array(0) <= skid_reg;
            skid := '0';
          end if;
        end if;

        pipeline_reg_gen: for i in 1 to PIPELINE_STAGES-1  loop
          if to_x01(rst) /= '0' then
            stage_in_array(i).valid   <= '0';
          elsif pr = '1' then
            stage_in_array(i)    <= stage_out_array(i-1);
          end if;
        end loop pipeline_reg_gen;
        
        if pr = '1' then
          out_reg := stage_out_array(PIPELINE_STAGES-1);
        end if;

        -- Interfacing
        dd_ready <= pr;

        output_valid <= out_reg.valid;
        output_data  <= out_reg.bin;
        output_last  <= out_reg.last;
        output_strb  <= not out_reg.empty;
      end if;  
    end process;

  stage_gen: for i in 0 to PIPELINE_STAGES-1  generate
    dd_stage(stage_in_array(i), stage_out_array(i), BITWIDTH, BITWIDTH/PIPELINE_STAGES);
  end generate stage_gen;
end behav;