use super::{Generator, components::{JsonComponent, JsonComponentValue}, til, types::{TilComponent, TilInlineImplementation, TilImplementationType, TilStreamType, Synchronicity, TilStreamParam}};

/**********************************************************************************
 * Set of functions to generate VHDL code around the components                   *
 **********************************************************************************/

 fn generate_prelude() -> String {
    let mut prelude = String::new();
    prelude.push_str("namespace schemaparser {\n\t");

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

        // Prepare the top component
        let top_component = self.analyze_from_top_component();

        // Register the top component
        self.gen_tools.entity_manager.register_top(top_component);

        // Generate the type definitions
        let type_defs = self.gen_tools.type_manager.generate_type_defs();
        til.push_str(&type_defs);

        // Generate the stream definitions
        let stream_defs = self.gen_tools.entity_manager.generate_stream_defs();
        til.push_str(&stream_defs);

        til.push_str(&til::generate_postlude());

        til
    }

    fn analyze_from_top_component(&mut self) -> TilComponent {
        let mut top_component = TilComponent::new("Top");

        let top_input_type = TilStreamType::new(
            "TopInStream",
            TilStreamParam::new(
                self.gen_params.bit_width,
                self.gen_params.epc,
                2,
                Synchronicity::Sync,
                8
            )
        );

        // Create input stream for the top component
        top_component.streams.add_input_stream("input", top_input_type.clone());
        self.gen_tools.type_manager.register(top_input_type);

        // Check if there is a root component
        if let Some(root) = &self.root {
            // Create the implementation of the top component
            let mut implementation = TilInlineImplementation::new();            

            // In-place traversal of the component tree
            let mut stack: Vec<JsonComponent> = Vec::new();

            // Add the root component to the stack
            stack.push(root.to_owned());

            // If the parent name is None, it means that the parent is the input stream
            let parent_name: Option<String> = None; 

            while !stack.is_empty() {
                let current_component = stack.pop().unwrap();

                // Generate TIL for the current component
                if let Some(ref component) = current_component.get_if_generatable() {
                    // Register the component as an entity
                    let til_comp = self.gen_tools.entity_manager.register(component, &self.gen_params, &mut self.gen_tools.type_manager);

                    // Add the component definition to the top component
                    let inst_name = implementation.add_instance(til_comp.get_name().to_string());

                    implementation.add_multiple_signals(
                        component.get_signals(&Some(inst_name.clone()), &parent_name)
                    );

                    // If the current component is a matcher
                    // Generate a matcher via the match manager
                    if let JsonComponent::Matcher(matcher) = &current_component {
                        self.gen_tools.match_manager.add_matcher(matcher.get_matcher(), &self.gen_params);
                    }

                    if component.num_outgoing_signals() == 0 {
                        // If the current component is a leaf, add the output stream to the top component
                        for stream in til_comp.streams.get_output_streams() {
                            let output_name = format!("output_{}", inst_name);
                            top_component.streams.add_output_stream(&output_name, stream.get_type().clone());
                        }
                    }
                }                

                // Add the children of the current component to the stack
                for child in current_component.get_children() {
                    stack.push(child);
                }
            }

            // Set the implementation of the top component
            top_component.set_implementation(TilImplementationType::Inline(implementation));
        }

        

        top_component
    }
}