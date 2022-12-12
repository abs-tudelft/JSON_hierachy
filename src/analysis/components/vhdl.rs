use indoc::formatdoc;

use super::{JsonComponent, JsonType};

/**********************************************************************************
 * Implementation of how a component is translated to VHDL                        *
 **********************************************************************************/

impl JsonComponent {
    pub fn to_vhdl(&self) -> String {
        match self {
            JsonComponent::Value { data_type, outer_nested } => {
                let mut vhdl = String::new();
                
                vhdl.push_str(&
                    match data_type {
                        JsonType::String => formatdoc!(
                            "
                            string_parser: JsonStrValParser
                            generic map (
                                EPC                       => EPC,
                                NESTING_LEVEL             => {} 
                            )
                            port map (
                                clk                       => clk,
                                reset                     => reset,
                                in_valid                  => in_valid,
                                in_ready                  => in_ready,
                                in_data                   => in_data,
                                in_last                   => in_last,
                                in_strb                   => in_strb,
                                out_data                  => out_data,
                                out_valid                 => out_valid,
                                out_ready                 => out_ready
                            );
                            ", outer_nested),
                        JsonType::Integer => formatdoc!(
                            "
                            int_parser: IntParser
                            generic map (
                                EPC                       => EPC,
                                NESTING_LEVEL             => {},
                                BITWIDTH                  => INT_WIDTH,
                                PIPELINE_STAGES           => INT_P_PIPELINE_STAGES
                            )
                            port map (
                                clk                       => clk,
                                reset                     => reset,
                                in_valid                  => array_valid,
                                in_ready                  => array_ready,
                                in_data                   => array_data,
                                in_last                   => array_last,
                                in_strb                   => array_strb,
                                out_data                  => out_data,
                                out_valid                 => out_valid,
                                out_ready                 => out_ready,
                                out_last                  => out_last,
                                out_strb                  => out_strb
                            );
                            ", outer_nested),
                        JsonType::Boolean => formatdoc!(
                            "
                            bool_parser: BooleanParser
                            generic map (
                                EPC                       => EPC,
                                NESTING_LEVEL             => {} 
                            )
                            port map (
                                clk                       => clk,
                                reset                     => reset,
                                in_valid                  => in_valid,
                                in_ready                  => in_ready,
                                in_data                   => in_data,
                                in_last                   => in_last,
                                in_strb                   => in_strb,
                                out_data                  => out_data,
                                out_valid                 => out_valid,
                                out_ready                 => out_ready
                            );
                            ", outer_nested)
                    }
                );

                // Add trailing new line
                vhdl.push_str("\n");

                vhdl
            },
            JsonComponent::Array { outer_nested, inner_nested, value } => {
                let mut vhdl = String::new();
                vhdl.push_str(&formatdoc!(
                    "
                    array_parser: JsonArrayParser
                    generic map (
                        EPC                       => EPC,
                        OUTER_NESTING_LEVEL       => {},
                        INNER_NESTING_LEVEL       => {}
                    )
                    port map (
                        clk                       => clk,
                        reset                     => reset,
                        in_valid                  => filter_valid,
                        in_ready                  => filter_ready,
                        in_data                   => filter_data,
                        in_last                   => filter_last,
                        in_strb                   => filter_strb,
                        out_data                  => array_data,
                        out_valid                 => array_valid,
                        out_ready                 => array_ready,
                        out_last                  => array_last,
                        out_stai                  => array_stai,
                        out_endi                  => array_endi,
                        out_strb                  => array_strb
                    );
                    ", outer_nested, inner_nested));

                // Add trailing new line
                vhdl.push_str("\n");

                // Recursively generate VHDL for the child component, if it exists
                if let Some(value) = value {
                    vhdl.push_str(&value.to_vhdl());
                }
                
                vhdl
            },
            JsonComponent::Key { name, outer_nested, value } => {
                let mut vhdl = String::new();
                vhdl.push_str(&formatdoc!(
                    "
                    {}_kf: KeyFilter
                    generic map (
                        EPC                       => EPC,
                        OUTER_NESTING_LEVEL       => {}
                    )
                    port map (
                        clk                       => clk,
                        reset                     => reset,
                        in_valid                  => kv_valid,
                        in_ready                  => kv_ready,
                        in_data                   => kv_vec,
                        in_strb                   => kv_strb,
                        in_last                   => kv_last,
                        matcher_str_valid         => matcher_str_valid,
                        matcher_str_ready         => matcher_str_ready,
                        matcher_str_data          => matcher_str_data,
                        matcher_str_strb          => matcher_str_strb,
                        matcher_str_last          => matcher_str_last,
                        matcher_match_valid       => matcher_match_valid,
                        matcher_match_ready       => matcher_match_ready,
                        matcher_match_data        => matcher_match,
                        out_valid                 => filter_valid,
                        out_ready                 => filter_ready,
                        out_data                  => filter_data,
                        out_strb                  => filter_strb,
                        out_stai                  => filter_stai,
                        out_endi                  => filter_endi,
                        out_last                  => filter_last
                    );

                    regex_matcher: {}_matcher
                    generic map (
                        BPC                       => EPC
                    )
                    port map (
                        clk                       => clk,
                        reset                     => reset,
                        in_valid                  => matcher_str_valid,
                        in_ready                  => matcher_str_ready,
                        in_strb                   => matcher_str_strb,
                        in_data                   => matcher_str_data,
                        in_last                   => matcher_str_last,
                        out_valid                 => matcher_match_valid,
                        out_ready                 => matcher_match_ready,
                        out_data                  => matcher_match
                    );
                    ", name, outer_nested, name));

                // Add trailing new line
                vhdl.push_str("\n");
                
                // Recursively generate VHDL for the value of the record, if it exists
                if let Some(value) = value {
                    vhdl.push_str(&value.to_vhdl());
                }

                vhdl
            },
            JsonComponent::Object { outer_nested, inner_nested, records } => {
                let mut vhdl = String::new();
                vhdl.push_str(&formatdoc!(
                    "
                    record_parser: JsonRecordParser
                    generic map (
                        EPC                       => EPC,
                        OUTER_NESTING_LEVEL       => {},
                        INNER_NESTING_LEVEL       => {},
                        END_REQ_EN                  => END_REQ_EN
                    )
                    port map (
                        clk                         => clk,
                        reset                       => reset,
                        in_valid                    => in_valid,
                        in_ready                    => in_ready,
                        in_data                     => in_data,
                        in_strb                     => in_strb,
                        in_last                     => in_last,
                        in_stai                     => in_stai,
                        in_endi                     => in_endi,
                        out_data                    => kv_vec,
                        out_stai                    => kv_stai,
                        out_endi                    => kv_endi,
                        out_ready                   => kv_ready,
                        out_valid                   => kv_valid,
                        out_strb                    => kv_strb,
                        out_last                    => kv_last
                    );
                    ", outer_nested, inner_nested));

                // Add trailing new line
                vhdl.push_str("\n");

                // Recursively generate VHDL for child components
                for child in records {
                    vhdl.push_str(&child.to_vhdl());
                }

                vhdl
            },
        }
    }
}