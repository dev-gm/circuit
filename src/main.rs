use std::collections::HashMap;

trait Gate {
    fn inputs(&self) -> usize;

    fn outputs(&self) -> &Vec<usize>;

    fn calculate(&self, inputs: Vec<bool>) -> Result<Vec<bool>, ()>;
}

struct BuiltinGate {
    inputs: usize,
    outputs: usize,
    process: fn(inputs: Vec<bool>) -> Result<Vec<bool>, ()>,
}

impl Gate for BuiltinGate {
    fn inputs(&self) -> usize {
        self.inputs
    }

    fn outputs(&self) -> &Vec<usize> {
        vec![0]
    }

    fn calculate(&self, inputs: Vec<bool>) -> Result<Vec<bool>, ()> {
        (self.process)(inputs)
    }
}

struct NormalGate {
    inputs: usize,
    outputs: Vec<usize>,
    process: Vec<NormalGateProcess>,
}

impl Gate for NormalGate {
    fn inputs(&self) -> usize {
        self.inputs
    }

    fn outputs(&self) -> &Vec<usize> {
        &self.outputs
    }

    fn calculate(&self, inputs: Vec<bool>) -> Result<Vec<bool>, ()> {
        if inputs.len() != self.inputs {
            return Err(());
        }
        let mut vars = HashMap::new();
        for (i, input) in inputs.into_iter().enumerate() {
            vars.insert(i, input);
        }
        for mut i in 0..self.process.len() {
            match &self.process[i] {
                NormalGateProcess::Gate { inputs: prior_inputs, gate } => {
                    let mut gate_inputs = Vec::new();
                    for input in prior_inputs {
                        if let Some(var) = vars.get(&input) {
                            gate_inputs.push(*var);
                        } else {
                            return Err(());
                        }
                    }
                    if let Ok(outputs) = gate.calculate(gate_inputs) {
                        let gate_outputs = gate.outputs();
                        for (i, output) in outputs.into_iter().enumerate() {
                            vars.insert(gate_outputs[i], output);
                        }
                    }
                },
                NormalGateProcess::Goto(line) => i = *line,
            }
        }
        let mut out = Vec::new();
        for output in &self.outputs {
            if let Some(var) = vars.get(&output) {
                out.push(*var);
            } else {
                return Err(());
            }
        }
        Ok(out)
    }
}

enum NormalGateProcess {
    Gate {
        inputs: Vec<usize>,
        gate: Box<dyn Gate>,
    },
    Goto(usize),
}

fn main() {
    println!("Hello, world!");
}
