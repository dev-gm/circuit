use std::collections::HashMap;
use std::iter::FromIterator;
use crate::lexer::LexInstruction;

pub struct App {
    gates: HashMap<String, Box<dyn Gate>>,
}

impl App {
    pub fn with_defaults() -> Self {
        Self {
            gates: HashMap::from_iter(vec![
                ("AND".to_string(), Box::new(BuiltinGate::new(2, 1, |inputs| vec![inputs[0] && inputs[1]])) as Box<dyn Gate>),
                ("NOT".to_string(), Box::new(BuiltinGate::new(1, 1, |inputs| vec![!inputs[0]])) as Box<dyn Gate>),
            ].into_iter())
        }
    }

    pub fn from_with_defaults(lex_instructions: Vec<LexInstruction>) -> Result<Self, String> {
        let mut out = Self::with_defaults();
        let gates = &mut out.gates;
        let mut current: Option<(String, usize, Vec<Instruction>, Vec<usize>)> = None; // name, inputs, process, outputs
        for instruction in lex_instructions {
            match instruction {
                LexInstruction::Def { name, inputs } => {
                    if let Some(inner) = current {
                        gates.insert(inner.0, Box::new(DefinedGate::new(inner.1, inner.2, inner.3)))
                            .ok_or("Could not add gate to app".to_string())?;
                    }
                    if gates.get(&name).is_some() {
                        return Err(format!("Gate name {} already exists!", name));
                    } else if inputs == 0 {
                        return Err("Inputs cannot be 0!".to_string());
                    }
                    current = Some((name, inputs, Vec::new(), Vec::new()));
                },
                LexInstruction::Gate { inputs, gate: gate_name, outputs, label } => {
                    if let Some(inner) = &mut current {
                        inner.2.push(Instruction::new(
                            inputs,
                            gates.get(&gate_name).ok_or(format!("Gate {} does not exist", gate_name))?,
                            outputs,
                            label,
                        ));
                    } else {
                        return Err("Cannot declare instruction outside of gate definition!".to_string());
                    }
                },
                LexInstruction::Goto { goto, label } => {},
                LexInstruction::Out { outputs } => {},
            }
        }
        Ok(out)
    }
}

pub trait Gate {
    fn inputs(&self) -> usize;

    fn outputs(&self) -> &Vec<usize>;

    fn calculate(&self, inputs: Vec<bool>) -> Result<Vec<bool>, ()>;
}

pub struct BuiltinGate {
    inputs: usize,
    function: fn(inputs: Vec<bool>) -> Vec<bool>,
    outputs: usize,
}

impl BuiltinGate {
    pub fn new(inputs: usize, outputs: usize, function: fn(inputs: Vec<bool>) -> Vec<bool>) -> Self {
        Self {
            inputs,
            function,
            outputs,
        }
    }
}

impl Gate for BuiltinGate {
    fn inputs(&self) -> usize { self.inputs }

    fn outputs(&self) -> &Vec<usize> { &vec![self.outputs] }

    fn calculate(&self, inputs: Vec<bool>) -> Result<Vec<bool>, ()> {
        if self.inputs != inputs.len() {
            return Err(());
        }
        Ok((self.function)(inputs))
    }
}

pub struct Instruction {
    label: Option<String>,
    inputs: Vec<usize>,
    gate: &'static Box<dyn Gate>,
    outputs: Vec<usize>,
}

impl Instruction {
    pub fn new(inputs: Vec<usize>, gate: &'static Box<dyn Gate>, outputs: Vec<usize>, label: Option<String>) -> Self {
        Self { label, inputs, gate, outputs }
    }

    pub fn inputs(&self) -> &Vec<usize> {
        &self.inputs
    }

    pub fn outputs(&self) -> &Vec<usize> {
        &self.outputs
    }

    pub fn calculate(&self, vars: &mut HashMap<usize, bool>) -> Result<(), ()> {
        let mut inputs = Vec::new();
        for input in &self.inputs {
            inputs.push(*vars.get(input).ok_or(())?);
        }
        for (i, output) in self.gate.calculate(inputs)?.into_iter().enumerate() {
            vars.insert(self.outputs[i], output);
        }
        Ok(())
    }
}

pub struct DefinedGate {
    inputs: usize,
    process: Vec<Instruction>,
    outputs: Vec<usize>,
}

impl DefinedGate {
    pub fn new(inputs: usize, process: Vec<Instruction>, outputs: Vec<usize>) -> Self {
        Self {
            inputs,
            process,
            outputs,
        }
    }
}

impl Gate for DefinedGate {
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
        for instruction in &self.process {
            instruction.calculate(&mut vars)?;
        }
        let mut out = Vec::new();
        for output in &self.outputs {
            out.push(vars.remove(&output).ok_or(())?)
        }
        Ok(out)
    }
}
