use crate::analysis::{types::{TilComponent, streaming_interface::TilStreamDirection, stream_types::{StreamTypeDecl, StreamDim}, TilSignal, til_component::{TilInlineImplementation, TilImplementationType}}, components::JsonComponentValue};

use super::{Analyzer, AnalyzerError, type_manager::StreamType};

impl Analyzer {
    pub fn assemble_top_component(&mut self) -> Result<TilComponent, AnalyzerError> {
        let mut top_component = TilComponent::new("top");
        
        let input_stream_name = "input";
        top_component.get_streams_mut().add_stream(input_stream_name, TilStreamDirection::Input, 
            StreamTypeDecl::new(
                StreamType::Json,
                Some(StreamDim::new(None, 2, 0))
            )
        );
        self.type_manager.register(StreamType::Json);

        let mut implementation = TilInlineImplementation::default(); 

        for entry in &self.top_component.as_ref().ok_or(AnalyzerError::NoTop)?.get_children() {
            if let Some(gen_com) = entry.get_if_generatable() {
                implementation.add_signal(TilSignal::Input { source_stream_name: input_stream_name.to_string(), dest_inst_name: gen_com.get_instance_name().to_string(), dest_stream_name: "input".to_owned() });
            }
        }

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