use crate::content::Merge;
use crate::propagator::Procedure;
use crate::network::Network;
use std::rc::{Rc, Weak};
use std::hash::Hash;
use std::fmt::Debug;
use crate::util::CellID;
use crate::tms::{TruthManagementSystem, Context};
use crate::premise::Premise;
use crate::content_tms::TruthManagementStore;


impl<C, T> Network<C, T> 
    where T: Merge + Debug + Hash + Clone + PartialEq + Eq,
{
    pub fn propagator_binary_amb<U: Premise>(&mut self, a: CellID) {

        let prop = Procedure::Unary(Box::new(move |context: &Context<T, U>, _a| {
            let tms = context.system.borrow();
            let true_premise : U = Premise::make_hypothetical(true, a);
            let false_premise : U = Premise::make_hypothetical(false, a);
            //let reasons_against_true = tms..reasons_against_premise(&true_premise);
            //let reasons_against_false = tms.reasons_against_premise(&false_premise);

            //a
            1
        }));

        //self.make_propagator(prop, &[a], a);
    }
}


