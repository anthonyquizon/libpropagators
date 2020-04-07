use crate::content::State;
use crate::util::CellID;

pub enum Return<T> {
    //Nothing
    Pure(T),
    AlertAllPropagators(T)
}

pub enum Procedure<C, T> {
    Unary(Box<dyn Fn(&C, T) -> Return<T>>),
    Binary(Box<dyn Fn(&C, T, T) -> Return<T>>),
    //Ternary(Fn(&C, T, T, T) -> T),
}

pub struct Propagator<C, T> {
    pub label: String,
    procedure: Procedure<C, T>,
    inputs: Vec<CellID>,
    output: CellID
}

impl<C,  T> Propagator<C, T> {
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
impl<C, T: Clone + State> Propagator<C, T> {
    pub fn invoke<'a, F: Fn(&CellID) -> &'a T>(&'a self, context: &C, read: F) -> Option<(CellID, Return<T>)> {
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
                proc(context, inputs[0].clone())
            },
            Procedure::Binary(proc) => {
                proc(
                    context,
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
