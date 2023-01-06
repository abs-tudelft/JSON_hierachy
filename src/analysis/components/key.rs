use crate::analysis::{types::{TilStreamingInterface, TilSignal, streaming_interface::{Generic, GenericType, TilStreamDirection, TilStream}, stream_types::{StreamTypeDecl, StreamDim}}, GeneratorParams, analyzer::{type_manager::StreamType, file_manager::TemplateType}};

use super::{Key, Generatable, JsonComponent, Matcher, JsonComponentValue};

impl Key {
    pub fn new(name: &str, matcher: Matcher, outer_nested: usize, value: Option<Box<JsonComponent>>) -> Key {
        Key {
            name: name.to_string(),
            matcher,
            outer_nested,
            value
        }
    }
}

impl Generatable for Key {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {
        let mut interface = TilStreamingInterface::default();

        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "DIM";
        let dim = self.outer_nested + 1;
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(dim)));
        interface.add_generic(Generic::new("OUTER_NESTING_LEVEL", GenericType::Positive(self.outer_nested)));

        // Input type
        interface.add_stream("input", TilStreamDirection::Input, 
            StreamTypeDecl::new(
                StreamType::Record,
                Some(StreamDim::new(Some(dim_name.to_string()), dim, 0))
            )
        );

        // Matcher type
        interface.add_stream("matcher_str", TilStreamDirection::Output, 
            StreamTypeDecl::new(
                StreamType::MatcherStr,
                None
            )
        );
        interface.add_stream("matcher_match", TilStreamDirection::Input, 
            StreamTypeDecl::new(
                StreamType::MatcherMatch,
                None
            )
        );
        // Output type
        interface.add_stream("output", TilStreamDirection::Output, 
            StreamTypeDecl::new(
                StreamType::Json,
                Some(StreamDim::new(Some(dim_name.to_string()), dim, 0))
            )
        );      

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Record, StreamType::MatcherStr, StreamType::MatcherMatch, StreamType::Json]
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_outgoing_signals(&self) -> Vec<TilSignal> {
        let mut signals = vec![
            TilSignal::Intermediate { 
                source_inst_name: self.get_instance_name(), 
                source_stream_name: "matcher_str".to_owned(), 
                dest_inst_name: self.matcher.get_instance_name(), 
                dest_stream_name: "input".to_owned() 
            }
        ];

        // First check if there is a child
        match &self.value {
            Some(child) => {
                // If the child is an object, get the children of the object
                let children = match **child {
                    JsonComponent::Object(ref obj) => obj.get_children(),
                    _ => vec![(**child).clone()],
                };

                for child in children {
                    // Force the child to be generatable
                    let child =Box::<dyn Generatable>::from(child);
                    signals.push(
                        TilSignal::Intermediate { 
                            source_inst_name: self.get_instance_name(), 
                            source_stream_name: "output".to_owned(), 
                            dest_inst_name: child.get_instance_name(), 
                            dest_stream_name: "input".to_owned() 
                        }
                    );
                }
            },
            None => {
                let output_name = format!("output_{}", self.get_instance_name());

                signals.push(
                    TilSignal::Output { 
                        source_inst_name: self.get_instance_name(), 
                        source_stream_name: "output".to_owned(), 
                        dest_stream_name: output_name.clone(),
                        output_stream: TilStream::new(&output_name, TilStreamDirection::Output, 
                            StreamTypeDecl::new( 
                                StreamType::Json,
                                Some(StreamDim::new(None, self.outer_nested, 1))
                            ) 
                        )
                    }
                );
            }
        };
        
        signals
    }

    fn num_outgoing_signals(&self) -> usize {
        2
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_file_type(&self) -> TemplateType {
        TemplateType::Key
    }
}

impl JsonComponentValue for Key {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Key filter\nO: {}", self.outer_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        match &self.value {
            Some(child) => vec![JsonComponent::Matcher(self.matcher.clone()), *child.clone()],
            None => vec![JsonComponent::Matcher(self.matcher.clone())],
        }
    }

    fn num_children(&self) -> usize {
        match &self.value {
            Some(_) => 2,
            None => 1,
        }
    }
}