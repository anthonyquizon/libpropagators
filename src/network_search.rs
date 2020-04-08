use crate::propagator::{Procedure, Return};
use crate::network::Network;
use std::rc::Rc;
use crate::util::CellID;
use crate::context_tms::{TruthManagementContext, Action, Premise};
use crate::content_tms::{TruthManagementStore};
use crate::content_float::{Float};


impl<T> Network<TruthManagementContext, TruthManagementStore<T>>
{
    pub fn propagator_binary_amb(&mut self, tms_rc: Rc<TruthManagementContext>, cell_id: CellID) {
        let true_premise : T = Premise::make_hypothetical(true, cell_id);
        let false_premise : T = Premise::make_hypothetical(false, cell_id);

        let prop_constant = Procedure::Nullary(Box::new(move || {
            let tms_rc = Rc::clone(&tms_rc);

            let a = TruthManagementStore::new(&tms_rc, &[
                (Float::new(0.), &[
                    true_premise,
                    false_premise
                ])
            ]);

            Return::Pure(a)
        }));

        let prop_amb_choose = Procedure::Nullary(Box::new(move || {
            Return::Action(Action::AmbChoose(true_premise, false_premise))
        }));

        self.make_propagator(prop_constant, &[], cell_id);
        self.make_propagator(prop_amb_choose, &[], cell_id);
    }
}


