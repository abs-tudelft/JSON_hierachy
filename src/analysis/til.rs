use super::{Generator, til};

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

 fn generate_prelude() -> String {
    let mut prelude = String::new();
    prelude.push_str("namespace schema::parser {\n\t");

    prelude
}

fn generate_postlude() -> String {
    let mut postlude = String::new();
    postlude.push('}');

    postlude
}

/**********************************************************************************
 * Implementation of how a component is translated to TIL                        *
 **********************************************************************************/

impl Generator {
    pub fn generate_til(&mut self) -> String {
        let mut til = String::new();

        til.push_str(&til::generate_prelude());

        let (type_defs, stream_defs) = self.analyzer.get_definitions();

        for type_def in type_defs {
            til.push_str(&type_def.get_type_def_string(&self.gen_params));
        }

        for stream_def in stream_defs {
            til.push_str(&format!("{}\n\n", stream_def));
        }

        // // Prepare the top component
        // let top_component = self.analyze_from_top_component();

        // // Register the top component
        // self.gen_tools.entity_manager.register_top(top_component, 1);

        // // Generate the type definitions
        // let type_defs = self.gen_tools.type_manager.generate_type_defs(&self.gen_params);
        // til.push_str(&type_defs);

        // // Generate the stream definitions
        // let stream_defs = self.gen_tools.entity_manager.generate_stream_defs();
        // til.push_str(&stream_defs);

        til.push_str(&til::generate_postlude());

        til
    }

    // fn analyze_from_top_component(&mut self) -> TilComponent {
    //     let mut top_component = TilComponent::new("top");

    //     // Create input stream for the top component
    //     let top_input_type = StreamType::Json(StreamDim::new(None, 2, 0));
    //     self.gen_tools.type_manager.register(top_input_type.clone());
    //     top_component.get_streams_mut().add_stream("input", TilStreamDirection::Input, top_input_type);
        

    //     // Check if there is a root component
    //     if let Some(root) = &self.root {
    //         // Create the implementation of the top component
    //         let mut implementation = TilInlineImplementation::new();            

    //         // In-place traversal of the component tree
    //         let mut stack: Vec<TreeComponent> = Vec::new();

    //         // Add the root component to the stack
    //         stack.push(TreeComponent { parent_name: None, component: root.to_owned() }); 

    //         while !stack.is_empty() {
    //             let current_tree_comp = stack.pop().unwrap();
    //             let current_component = current_tree_comp.component;
    //             let parent_name = current_tree_comp.parent_name;
    //             let mut comp_name = parent_name.clone();

    //             // Generate TIL for the current component
    //             if let Some(ref component) = current_component.get_if_generatable() {
    //                 // Register the component as an entity
    //                 let til_comp = self.gen_tools.entity_manager.register(component, &self.gen_params, &mut self.gen_tools.type_manager, 0);

    //                 // Add the component definition to the top component
    //                 let inst_name = implementation.add_instance(til_comp.get_name().to_string());

    //                 implementation.add_multiple_signals(
    //                     match parent_name {
    //                         Some(_) => {
    //                             component.get_signals(&Some(inst_name.clone()), "input", &parent_name, "output")
    //                         },
    //                         None => {
    //                             component.get_signals(&Some(inst_name.clone()), "input", &None, "input")
    //                         }
    //                     }
                        
    //                 );

    //                 // If the current component is a matcher
    //                 // Generate a matcher via the match manager
    //                 if let JsonComponent::Matcher(matcher) = &current_component {
    //                     self.gen_tools.match_manager.add_matcher(matcher.get_matcher(), &self.gen_params);
    //                 }

    //                 if component.num_outgoing_signals() == 0 {
    //                     // If the current component is a leaf, add the output stream to the top component
    //                     for stream in til_comp.get_streams().get_output_streams() {
    //                         let output_name = format!("output_{}", inst_name);

    //                         // Copy the stream type without the generic name
    //                         let stream_type = match stream.get_type() {
    //                             StreamType::Json(stream_dim) => {
    //                                 StreamType::Json(StreamDim::new(None, stream_dim.get_value(), stream_dim.get_additive()))
    //                             },
    //                             StreamType::Bool(stream_dim) => {
    //                                 StreamType::Bool(StreamDim::new(None, stream_dim.get_value(), stream_dim.get_additive()))
    //                             },
    //                             StreamType::Int(stream_dim) => {
    //                                 StreamType::Int(StreamDim::new(None, stream_dim.get_value(), stream_dim.get_additive()))
    //                             },
    //                             StreamType::Record(stream_dim) => {
    //                                 StreamType::Record(StreamDim::new(None, stream_dim.get_value(), stream_dim.get_additive()))
    //                             },
    //                             _ => stream.get_type().clone()
    //                         };

    //                         top_component.get_streams_mut().add_stream(&output_name, TilStreamDirection::Output, stream_type);
    //                         implementation.add_signal(
    //                             TilSignal::new(
    //                                 &Some(inst_name.clone()), 
    //                                 "output", 
    //                                 &None, 
    //                                 &output_name
    //                             )
    //                         );
    //                     }
    //                 }

    //                 // Set the parent name to the current instance name
    //                 comp_name = Some(inst_name);
    //             }                

    //             // Add the children of the current component to the stack
    //             for child in current_component.get_children() {
    //                 stack.push(TreeComponent { parent_name: comp_name.clone(), component: child });
    //             }
    //         }

    //         // Set the implementation of the top component
    //         top_component.set_implementation(TilImplementationType::Inline(implementation));
    //     }

        

    //     top_component
    // }
}

// struct TreeComponent {
//     pub parent_name: Option<String>,
//     pub component: JsonComponent
// }