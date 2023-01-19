use crate::analysis::types::{TilStreamlets, streaming_interface::TilStreamDirection, stream_types::{StreamTypeDecl, StreamDim}, TilSignal, til_streamlets::{TilInlineImplementation, TilImplementationType}};

use super::{Analyzer, AnalyzerError, type_manager::StreamType};

impl Analyzer {
    pub fn assemble_top_component(&mut self) -> Result<TilStreamlets, AnalyzerError> {
        let mut top_component = TilStreamlets::new("top");
        
        let input_stream_name = "input";
        top_component.get_streams_mut().add_stream(input_stream_name, TilStreamDirection::Input, 
            StreamTypeDecl::new(
                StreamType::Json,
                Some(StreamDim::new(None, 2, 0))
            )
        );
        self.type_manager.register(StreamType::Json);

        let mut implementation = TilInlineImplementation::default(); 


        let gen_com = self.top_component.as_ref()
            // Check if it exists
            .ok_or(AnalyzerError::NoTop)?
            // Get as generatable
            .get_generatable();

        implementation.add_signal(TilSignal::Input { source_stream_name: input_stream_name.to_string(), dest_inst_name: gen_com.get_instance_name().to_string(), dest_stream_name: "input".to_owned() });

        let mut output_signals: Vec<TilSignal> = Vec::new();
        for signal in &self.signal_list {
            match signal {
                TilSignal::Intermediate { .. } => {
                    implementation.add_signal(signal.clone());
                },
                TilSignal::Output { output_stream, .. } => {
                    output_signals.push(signal.clone());
                    top_component.get_streams_mut().add_til_stream(output_stream.clone());
                },
                _ => {}
            }
        }

        implementation.add_multiple_signals(output_signals);

        for inst in &self.entity_list {
            implementation.add_instance(inst.get_name().to_string());
        }
        
        top_component.set_implementation(TilImplementationType::Inline(implementation));

        Ok(top_component)
    }
}