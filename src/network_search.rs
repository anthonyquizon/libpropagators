use crate::content::Merge;
use crate::propagator::{Procedure, Return};
use crate::network::Network;
use std::hash::Hash;
use std::fmt::Debug;
use std::rc::Rc;
use crate::util::CellID;
use crate::context::Context;
use crate::context_tms::{TruthManagementContext, Action};
use crate::content_tms::{TruthManagementStore};
use crate::content_float::{Float};
use crate::premise::Premise;



impl<U: Premise + 'static> Network<TruthManagementContext<U>, TruthManagementStore<Float, U>> 
{
    pub fn propagator_binary_amb(&mut self, tms_rc: Rc<TruthManagementContext<U>>, cell_id: CellID) {
        let prop_constant = Procedure::Nullary(Box::new(move || {
            let tms_rc = Rc::clone(&tms_rc);
            let true_premise : U = Premise::make_hypothetical(true, cell_id);
            let false_premise : U = Premise::make_hypothetical(false, cell_id);

            let a = TruthManagementStore::new(&tms_rc, &[
                (Float::new(0.), &[
                    true_premise,
                    false_premise
                ])
            ]);

            Return::Pure(a)
        }));

        let prop_amb_choose = Procedure::Nullary(Box::new(move || {
            Return::Action(Action::AmbChoose(cell_id))
        }));

        self.make_propagator(prop_constant, &[], cell_id);
        self.make_propagator(prop_amb_choose, &[], cell_id);
    }
}


