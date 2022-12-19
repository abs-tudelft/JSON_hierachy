use super::{Generator, components::{JsonComponent, JsonComponentValue, Matcher}, til, types::{TilComponent, TilInlineImplementation, TilImplementationType}};

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
        match &self.root {
            Some(root) => {
                let mut til = String::new();

                til.push_str(&til::generate_prelude());

                // Prepare the generation by analyzing the component tree
                let top_component = self.analyze_from_top_component();

                let type_defs = self.gen_tools.type_manager.generate_type_defs();
                til.push_str(&type_defs);

                let stream_defs = self.gen_tools.entity_manager.generate_stream_defs();
                til.push_str(&stream_defs);

                
                // let comp_til = self.til_from_component(top_component);

                til.push_str(&til::generate_postlude());

                til
            },
            None => String::from(""),
        }
    }

    fn analyze_from_top_component(&mut self) -> TilComponent {
        let mut top_component = TilComponent::new("Top");

        // Check if there is a root component
        if let Some(root) = &self.root {
            // Create the implementation of the top component
            let mut implementation = TilInlineImplementation::new();            

            // In-place traversal of the component tree
            let mut stack: Vec<JsonComponent> = Vec::new();

            // Add the root component to the stack
            stack.push(root.to_owned());

            while !stack.is_empty() {
                let current_component = stack.pop().unwrap();

                // Generate TIL for the current component
                if let Some(ref component) = current_component.get_if_generatable() {
                    // Register the component as an entity
                    let comp_def = self.gen_tools.entity_manager.register(component, &self.gen_params, &mut self.gen_tools.type_manager);

                    // Add the component definition to the top component
                    implementation.add_instance(comp_def);
                }

                // If the current component is a matcher, generate a matcher via the match manager
                if let JsonComponent::Matcher(matcher) = &current_component {
                    self.gen_tools.match_manager.add_matcher(matcher.get_matcher(), &self.gen_params);
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