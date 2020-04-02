use crate::util::CellID;
use crate::content::State;

pub enum Procedure<T> {
    Unary(fn(T) -> T),
    Binary(fn(T, T) -> T),
    Ternary(fn(T, T, T) -> T),
}

pub struct Propagator<T> {
    pub label: String,
    procedure: Procedure<T>,
    inputs: Vec<CellID>,
    output: CellID
}

impl<T> Propagator<T> {
    pub fn new(procedure: Procedure<T>, inputs: &[CellID], output: CellID) -> Self {
        Self {
            label: String::from(""),
            procedure,
            inputs: inputs.into(),
            output
        }
    }
}

//FIXME: remove clone
impl<T: Clone + State> Propagator<T> {
    pub fn invoke<'a, F: Fn(&CellID) -> &'a T>(&'a self, read: F) -> Option<(CellID, T)> {
        let inputs : Vec<T> = self.inputs
            .iter()
            .map(read)
            .cloned() //FIXME remove cloned
            .collect(); 

        if inputs.iter().any(|cell| cell.is_empty()) {
            return None;
        }

        //FIXME: remove clones
        let output = match &self.procedure {
            Procedure::Unary(proc) => {
                proc(inputs[0].clone())
            },
            Procedure::Binary(proc) => {
                proc(
                    inputs[0].clone(), 
                    inputs[1].clone()
                )
            },
            Procedure::Ternary(proc) => {
                proc(
                    inputs[0].clone(), 
                    inputs[1].clone(),
                    inputs[2].clone()
                )
            }
        };

        Some((self.output, output))
    }
}
