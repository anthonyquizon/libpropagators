use crate::content::Merge;
use crate::propagator::{Procedure, Return};
use crate::network::Network;
use std::hash::Hash;
use std::fmt::Debug;
use crate::util::CellID;
use crate::context_tms::TruthManagementSystem;
use crate::premise::Premise;


impl<C, T> Network<C, T> 
    where T: Merge + Debug + Hash + Clone + PartialEq + Eq,
{
    pub fn propagator_binary_amb<U: Premise>(&mut self, a: CellID) {

        let prop = Procedure::Unary(Box::new(move |tms: &TruthManagementSystem<T>, _a| {
            let true_premise : U = Premise::make_hypothetical(true, a);
            let false_premise : U = Premise::make_hypothetical(false, a);
            //let reasons_against_true = tms..reasons_against_premise(&true_premise);
            //let reasons_against_false = tms.reasons_against_premise(&false_premise);

            //a
            Return::Pure(1)
        }));

        //self.make_propagator(prop, &[a], a);
    }
}


