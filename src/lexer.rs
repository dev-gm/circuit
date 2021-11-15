use std::fs::File;
use std::path::PathBuf;
use std::io::{
    self,
    Read,
};
use std::str::FromStr;

pub struct LexGate {
    name: String,
    inputs: usize,
    gates: Vec<LexGateInstruction>,
    outputs: Vec<usize>,
}

pub enum LexGateInstruction {
    Gate {
        inputs: Vec<usize>,
        gate: String,
        outputs: Vec<usize>,
    },
    Goto {
        label: String,
    }
}

pub enum LexInstruction {
    Gate {
        inputs: Vec<usize>,
        gate: String,
        outputs: Vec<usize>,
        label: Option<String>,
    },
    Goto {
        goto: String,
        label: Option<String>,
    },
    Def { // gate name
        name: String,
        inputs: usize,
    },
    Out {
        outputs: Vec<usize>,
    }
}

impl LexInstruction {
    pub fn new_gate(inputs: Vec<usize>, gate: String, outputs: Vec<usize>, label: Option<String>) -> Self {
        Self::Gate {
            inputs,
            gate,
            outputs,
            label,
        }
    }

    pub fn new_goto(goto: String, label: Option<String>) -> Self {
        Self::Goto { goto, label }
    }

    pub fn new_def(name: String, inputs: usize) -> Self {
        Self::Def { name, inputs }
    }
/*
    pub fn new_in(inputs: usize) -> Self {
        Self::In { inputs }
    }
*/
    pub fn new_out(outputs: Vec<usize>) -> Self {
        Self::Out { outputs }
    }
}

impl FromStr for LexInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let line: Vec<&str> = input // {label};{AND}:{1,2}->{3}
            .split(';')
            .map(|s| s
                .split(':')
                .map(|s| s
                    .split("->")
                )
                .flatten()
            )
            .flatten()
            .collect();
        let (mut label, mut gate, mut inputs, mut output) = (None, None, None, None);
        if input.contains(';') {
            label = Some(line[0].to_string());
            gate = Some(line[1].to_string());
            inputs = Some(line[2].to_string());
        } else {
            gate = Some(line[0].to_string());
            inputs = Some(line[1].to_string());
        }
        let inputs = inputs.ok_or(())?;
        if input.contains("->") {
            output = Some(line.last().ok_or(())?.to_string());
        }
        Ok(match line[0].to_uppercase().as_str() {
            "GOTO" => Self::new_goto(inputs, label),
            "DEF" => Self::new_def(inputs, output.ok_or(())?.parse().or(Err(()))?),
            "OUT" => Self::new_out(str_to_vec(inputs, ',')?),
            _ => Self::new_gate(
                str_to_vec(inputs, ',')?,
                gate.ok_or(())?,
                str_to_vec(output.ok_or(())?, ',')?,
                label,
            ),
        })
    }
}

trait InstructionLexer {
    type Reader: Read;

    fn add_reader(&mut self, reader: Self::Reader);

    fn reader(&mut self) -> &mut Self::Reader;

    fn content(&mut self) -> Result<String, io::Error> {
        let mut out = String::new();
        self.reader().read_to_string(&mut out)?;
        Ok(out)
    }

    fn instructions(&mut self) -> Result<Vec<LexInstruction>, ()>;
}

pub struct FileInstructionLexer {
    file: File,
}

impl FileInstructionLexer {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        Ok(Self { file: File::open(path)? })
    }
}

impl InstructionLexer for FileInstructionLexer {
    type Reader = File;

    fn add_reader(&mut self, reader: Self::Reader) {
        self.file = reader;
    }

    fn reader(&mut self) -> &mut Self::Reader {
        &mut self.file
    }

    fn instructions(&mut self) -> Result<Vec<LexInstruction>, ()> {
        let mut instructions = Vec::new();
        let content = self.content().or(Err(()))?;
        let lines: Vec<&str> = content
            .split('\n')
            .collect();
        for line in lines {
            instructions.push(LexInstruction::from_str(line)?);
        }
        Ok(instructions)
    }
}

fn str_to_vec<T: FromStr>(input: String, delim: char) -> Result<Vec<T>, ()> {
    let mut out = Vec::new();
    let results: Vec<_> = input
        .split(delim)
        .map(|s| s.parse::<T>())
        .collect();
    for result in results {
        out.push(result.or(Err(()))?);
    }
    Ok(out)
}
