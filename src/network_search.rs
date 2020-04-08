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



impl Network<TruthManagementContext<String>, TruthManagementStore<Float, String>> 
{
    pub fn propagator_binary_amb<U: Premise>(&mut self, tms_rc: Rc<TruthManagementContext<String>>, cell_id: CellID) {
        let true_premise : U = Premise::make_hypothetical(true, cell_id);
        let false_premise : U = Premise::make_hypothetical(false, cell_id);

        let prop = Procedure::Nullary(Box::new(move || {
            let tms_rc = Rc::clone(&tms_rc);
            let a = TruthManagementStore::new(&tms_rc, &[
                (Float::new(1.), &[String::from("foo")])
            ]);

            Return::Pure(a)
        }));

        self.make_propagator(prop, &[cell_id], cell_id);
    }
}


