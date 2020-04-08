use crate::network::{ Network };
use crate::premise::Premise;
use crate::context::Context;
use crate::util::CellID;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;


pub enum Action {
    AmbChoose(CellID)
}

#[derive(Clone)]
pub struct TruthManagementContext<T> {
    premise_outness: HashSet<T>,
    premise_nogoods: HashMap<T, Vec<(T, T)>>,
}

impl<T> fmt::Debug for TruthManagementContext<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Truth Management System")

        //TODO premises
    }
}

impl<T: Premise> TruthManagementContext<T> {
    pub fn new() -> Self {
        Self {
            premise_outness: HashSet::new(),
            premise_nogoods: HashMap::new()
        }
    }

    pub fn premise_in(&self, premise: &T) -> bool {
        !self.premise_outness.contains(premise)
    }

    //fn pairwise_union() {
        //let mut vec = Vec:new();

        //for 
    //}
}

impl<T: Premise> Context for TruthManagementContext<T> {
    type Action = Action;

    fn run_action(&mut self, action: Self::Action) {
        match action {
            Action::AmbChoose(cell_id) => {
                let true_premise : T = Premise::make_hypothetical(true, cell_id);
                let false_premise : T = Premise::make_hypothetical(false, cell_id);

                let true_no_goods = self.premise_nogoods.get(&true_premise);
                let false_no_goods = self.premise_nogoods.get(&false_premise);

                let reasons_agains_true = match true_no_goods {
                    Some(no_goods) => {
                        no_goods.iter().filter(|premises| {
                            true
                        }).collect()
                    },
                    None => Vec::new()
                };
            }
        }
    }

}
