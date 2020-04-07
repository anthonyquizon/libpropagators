use crate::content::Merge;
use crate::propagator::{Procedure, Return};
use crate::network::Network;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;
use crate::util::CellID;
use crate::context::Context;
use crate::context_tms::{TruthManagementSystem, TruthManagementContext};
use crate::premise::Premise;


impl<C: TruthManagementSystem, T> Network<C, T> 
    where T: Merge + Debug + Hash + Clone + PartialEq + Eq,
{
    pub fn propagator_binary_amb<U: Premise>(&mut self, cell_id: CellID) {
        let prop = Procedure::Unary(Box::new(move |tms: &C, value: T| {
            let true_premise : U = Premise::make_hypothetical(true, cell_id);
            let false_premise : U = Premise::make_hypothetical(false, cell_id);


            //let reasons_against_true = tms..reasons_against_premise(&true_premise);
            //let reasons_against_false = tms.reasons_against_premise(&false_premise);

            //a
            Return::Pure(value)
        }));

        self.make_propagator(prop, &[cell_id], cell_id);
    }
}


