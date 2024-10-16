use std::fmt::{Display, Formatter};

use indoc::{formatdoc, writedoc};

use super::{TilStreamlet, TilStreamingInterface, TilSignal};

impl TilStreamlet {
    pub fn new(name: &str) -> TilStreamlet {
        TilStreamlet {
            name: String::from(name),
            streams: TilStreamingInterface::default(),
            implementation: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_implementation(&mut self, implementation: TilImplementationType) {
        self.implementation = Some(implementation);
    }

    pub fn set_streaming_interface(&mut self, stream_interface: TilStreamingInterface) {
        self.streams = stream_interface;
    }

    pub fn get_streams(&self) -> &TilStreamingInterface {
        &self.streams
    }

    pub fn get_streams_mut(&mut self) -> &mut TilStreamingInterface {
        &mut self.streams
    }

    pub fn get_implementation(&self) -> &Option<TilImplementationType> {
        &self.implementation
    }

    pub fn td (&self) -> String {
        let mut comp_def = String::new();
        let mut generics = String::new();
        let mut stream_defs = String::new();

        if !self.get_streams().get_generics().is_empty() {
            let mut generic_defs = "\n".to_string();

            let str = self.get_streams().get_generics().iter().map(|e| e.td()).collect::<Vec<_>>().join("");
            generic_defs.push_str(&str);

            generics = generic_defs;
        }

        for stream in self.get_streams().get_streams() {
            stream_defs.push_str(
                &format!("    {}\n", stream.td())
            );
        }

        comp_def.push_str(
            &formatdoc!(
                "streamlet {} {{{generics}\n{stream_defs}}}",
                self.get_name(),
            )
        );

        if let Some(implementation) = &self.implementation {
            let temp_str = implementation.td(self.get_name().to_string());
            comp_def += &temp_str;
        }

        comp_def
    }
}

impl Display for TilStreamlet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut comp_def = String::new();
        let mut generics = String::new();
        let mut stream_defs = String::new();

        if !self.get_streams().get_generics().is_empty() {
            let mut generic_defs = String::new();

            for generic in self.get_streams().get_generics() {
                generic_defs.push_str(
                    &format!("{},\n", generic)
                );
            }

            generics = formatdoc!(
                "
                <
                {}
                >",
                generic_defs
            );
        }

        for stream in self.get_streams().get_streams() {
            stream_defs.push_str(
                &format!("    {},\n", stream)
            );
        }

        comp_def.push_str(
            &formatdoc!(
                "
                streamlet {} = {} (
                    {}
                )",
                self.get_name(),
                generics,
                stream_defs
            )
        );

        if let Some(implementation) = &self.implementation {
            comp_def.push_str(&implementation.to_string());
        }

        write!(f, "{};", comp_def)
    }
}

#[derive(Clone)]
pub enum TilImplementationType {
    Path(String),
    Inline(TilInlineImplementation),
}

impl TilImplementationType {
    pub fn td(&self, name: String) -> String {
        match self {
            TilImplementationType::Inline(inline) => {
                let mut impl_til = String::new();

                for instance in inline.get_instances() {
                    impl_til.push_str(&instance.td());
                    impl_til.push('\n');
                }

                impl_til.push_str("\n    ");

                for signal in inline.get_signals() {
                    impl_til.push_str(&signal.td());
                    impl_til.push('\n');
                }
                
                format!("\n\nimpl {name}_impl of {name} {{\n{impl_til}}}")
            },
            TilImplementationType::Path(path) => {
                format!("\n\nimpl {name}_impl of {name} @External {{ }}")
            }
        }
    }
}

#[derive(Clone, Default)]
pub struct TilInlineImplementation {
    instances: Vec<TilInstance>,
    signals: Vec<TilSignal>,
}

impl TilInlineImplementation {
    pub fn add_instance(&mut self, component_name: String) -> String{
        let instance_name = format!("{}_inst", component_name);
        self.instances.push(TilInstance::new(&component_name, &instance_name));
        instance_name
    }

    pub fn add_signal(&mut self, signal: TilSignal) {
        self.signals.push(signal);
    }

    pub fn add_multiple_signals(&mut self, signals: Vec<TilSignal>) {
        self.signals.extend(signals);
    }

    pub fn get_instances(&self) -> &Vec<TilInstance> {
        &self.instances
    }

    pub fn get_signals(&self) -> &Vec<TilSignal> {
        &self.signals
    }
}

impl Display for TilImplementationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TilImplementationType::Inline(inline) => {
                let mut impl_til = String::new();

                for instance in inline.get_instances() {
                    impl_til.push_str(&instance.to_string());
                    impl_til.push('\n');
                }

                impl_til.push('\n');

                for signal in inline.get_signals() {
                    impl_til.push_str(&signal.to_string());
                    impl_til.push('\n');
                }

                
                writedoc!(f,
                    "
                    {{
                        impl: {{
                            {}
                        }}
                    }}",
                    impl_til
                )
            },
            TilImplementationType::Path(path) => {
                writedoc!(f,
                    "
                    {{
                        impl: \"{}\"
                    }}",
                    path
                )
            }
        }
    }
}

#[derive(Clone)]
pub struct TilInstance {
    component_name: String,
    instance_name: String,
}

impl TilInstance {
    pub fn new(component_name: &str, instance_name: &str) -> TilInstance {
        TilInstance {
            component_name: String::from(component_name),
            instance_name: String::from(instance_name),
        }
    }

    pub fn td(&self) -> String {
        format!("    instance {}({}_impl);", self.instance_name, self.component_name)
    }
}

impl Display for TilInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {};", self.instance_name, self.component_name)
    }
}