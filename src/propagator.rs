use crate::content::State;
use crate::util::CellID;
use crate::context::Context;

pub enum Return<C: Context, T> {
    Pure(T),
    Action(C::Action),
    //TODO action with output
}

pub enum Procedure<C: Context, T> {
    Nullary(Box<dyn Fn() -> Return<C, T>>),
    Unary(Box<dyn Fn(T) -> Return<C, T>>),
    Binary(Box<dyn Fn(T, T) -> Return<C, T>>),
    //Ternary(Fn(&C, T, T, T) -> T),
}

pub struct Propagator<C: Context, T> {
    pub label: String,
    procedure: Procedure<C, T>,
    inputs: Vec<CellID>,
    output: CellID
}

impl<C: Context,  T> Propagator<C, T> {
    pub fn new(procedure: Procedure<C, T>, inputs: &[CellID], output: CellID) -> Self {
        Self {
            label: String::from(""),
            procedure,
            inputs: inputs.into(),
            output
        }
    }
}

//FIXME: remove clone
impl<C: Context, T: Clone + State> Propagator<C, T> {
    pub fn invoke<'a, F: Fn(&CellID) -> &'a T>(&'a self, read: F) -> Option<(CellID, Return<C, T>)> {
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
            Procedure::Nullary(proc) => {
                proc()
            },
            Procedure::Unary(proc) => {
                proc(inputs[0].clone())
            },
            Procedure::Binary(proc) => {
                proc(
                    inputs[0].clone(), 
                    inputs[1].clone()
                )
            },
            //Procedure::Ternary(proc) => {
                //proc(
                    //context,
                    //inputs[0].clone(), 
                    //inputs[1].clone(),
                    //inputs[2].clone()
                //)
            //}
        };

        Some((self.output, output))
    }
}
