use crate::analysis::types::{TilStreamlet, streaming_interface::TilStreamDirection, stream_types::{StreamTypeDecl, StreamDim}, TilSignal, til_streamlet::{TilInlineImplementation, TilImplementationType}};

use super::{Analyzer, AnalyzerError, type_manager::StreamType};

impl Analyzer {
    pub fn assemble_top_component(&mut self) -> Result<TilStreamlet, AnalyzerError> {
        let mut top_component = TilStreamlet::new("top");
        
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

        implementation.add_signal(TilSignal::Input { source_stream_name: input_stream_name.to_string(), dest_inst_name: gen_com.get_instance_name(), dest_stream_name: "input".to_owned() });

        // Add all intermediate signals to the implementation
        implementation.add_multiple_signals(self.signal_manager.get_intermediate_signals().to_vec());

        // Add all output signals to the implementation and add the output streams to the top component respectively
        for signal in self.signal_manager.get_output_signals() {
            // Get the output stream to add it to the top component
            match signal {
                TilSignal::Output { output_stream, .. } => {
                    top_component.get_streams_mut().add_til_stream(output_stream.clone());
                },
                _ => panic!("None output signal in output signal list") // This should never happen
            }

            // Add the signal to the implementation
            implementation.add_signal(signal.clone());
        }

        for inst in &self.entity_list {
            implementation.add_instance(inst.get_name().to_string());
        }
        
        top_component.set_implementation(TilImplementationType::Inline(implementation));

        Ok(top_component)
    }
}