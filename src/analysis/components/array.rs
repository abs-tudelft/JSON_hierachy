use crate::analysis::{types::{TilStreamingInterface, streaming_interface::{Generic, GenericType, TilStreamDirection, TilStream}, stream_types::{StreamTypeDecl, StreamDim}, TilSignal}, GeneratorParams, analyzer::{type_manager::StreamType, file_manager::TemplateType}};

use super::{Array, JsonComponent, Generatable, JsonComponentValue};

impl Array {
    pub fn new(name: &str, outer_nested: usize, inner_nested: usize, value: Option<Box<JsonComponent>>) -> Array {
        Array {
            name: name.to_string(),
            outer_nested,
            inner_nested,
            value,
        }
    }
}

impl Generatable for Array {
    fn get_streaming_interface(&self, gen_params: &GeneratorParams) -> TilStreamingInterface {       
        let mut interface = TilStreamingInterface::default();

        interface.add_generic(Generic::new("EPC", GenericType::Positive(gen_params.epc)));
        let dim_name = "DIM";
        let dim = self.outer_nested + 1;
        interface.add_generic(Generic::new(dim_name, GenericType::Dimensionality(dim)));
        interface.add_generic(Generic::new("OUTER_NESTING_LEVEL", GenericType::Natural(self.outer_nested)));
        interface.add_generic(Generic::new("INNER_NESTING_LEVEL", GenericType::Natural(self.inner_nested)));
        

        // Input type
        interface.add_stream("input", TilStreamDirection::Input,
            StreamTypeDecl::new( 
                StreamType::Json,
                Some(StreamDim::new(Some(dim_name.to_string()), dim, 0))
            )
        );

        // Output type
        interface.add_stream("output", TilStreamDirection::Output,
            StreamTypeDecl::new( 
                StreamType::Json,
                Some(StreamDim::new(Some(dim_name.to_string()), dim, 1))
            )
        );

        interface
    }

    fn get_streaming_types(&self) -> Vec<StreamType> {
        vec![StreamType::Json]
    }

    fn get_nesting_level(&self) -> usize {
        self.outer_nested
    }

    fn get_outgoing_signals(&self) -> Vec<TilSignal> {
        // First check if there is a child
        match &self.value {
            Some(child) => {

                // If the child is an object, get the children of the object
                let children = match **child {
                    JsonComponent::Object(ref obj) => obj.get_children(),
                    _ => vec![(**child).clone()],
                };

                let mut signals: Vec<TilSignal> = Vec::new();

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

                signals
            },
            None => {
                let output_name = format!("output_{}", self.get_instance_name());

                vec![
                    TilSignal::Output { 
                        source_inst_name: self.get_instance_name(), 
                        source_stream_name: "output".to_owned(), 
                        dest_stream_name: output_name.clone(),
                        output_stream: TilStream::new(&output_name, TilStreamDirection::Output, 
                            StreamTypeDecl::new( 
                                StreamType::Json,
                                Some(StreamDim::new(None, self.outer_nested, 2))
                            ) 
                        )
                    }
                ]
            },
        }       
    }

    fn num_outgoing_signals(&self) -> usize {
        1
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_file_type(&self) -> TemplateType {
        TemplateType::Array
    }
}

impl JsonComponentValue for Array {
    fn to_graph_node(&self) -> Option<String> {
        Some(
            format!("Array parser\nO: {}, I: {}", self.outer_nested, self.inner_nested)
        )
    }

    fn get_children(&self) -> Vec<JsonComponent> {
        match &self.value {
            Some(child) => vec![*child.clone()],
            None => vec![],
        }
    }

    fn num_children(&self) -> usize {
        match &self.value {
            Some(_) => 1,
            None => 0,
        }
    }
}