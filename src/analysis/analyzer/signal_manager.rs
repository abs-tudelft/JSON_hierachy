use crate::analysis::types::TilSignal;

#[derive(Default)]
pub struct SignalManager {
    intermediate_signals: Vec<TilSignal>,
    output_signals: Vec<TilSignal>,
}

impl SignalManager {
    pub fn add_signal(&mut self, signal: TilSignal) {
        match signal {
            TilSignal::Intermediate { .. } => {
                self.intermediate_signals.push(signal);
            },
            TilSignal::Output { .. } => {
                self.output_signals.push(signal);
            },
            _ => {}
        }
    }

    pub fn add_multiple_signals(&mut self, signals: Vec<TilSignal>) {
        for signal in signals {
            self.add_signal(signal);
        }
    }

    pub fn get_intermediate_signals(&self) -> &Vec<TilSignal> {
        &self.intermediate_signals
    }

    pub fn get_output_signals(&self) -> &Vec<TilSignal> {
        &self.output_signals
    }
}
