use crate::context::Context;
use std::collections::HashSet;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;


pub enum Action<Premise> {
    AmbChoose(Premise, Premise)
}

#[derive(Clone)]
pub struct TruthManagementContext<Premise> {
    premise_outness: HashSet<Premise>,
    premise_nogoods: HashMap<Premise, Vec<(Premise, Premise)>>,
}

impl<Premise: Debug> fmt::Debug for TruthManagementContext<Premise> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Truth Management System")
        //TODO premises
    }
}

impl<Premise> TruthManagementContext<Premise> {
    pub fn new() -> Self {
        Self {
            premise_outness: HashSet::new(),
            premise_nogoods: HashMap::new()
        }
    }

    pub fn premise_in(&self, premise: &Premise) -> bool {
        !self.premise_outness.contains(premise)
    }
}

impl<Premise> Context for TruthManagementContext<Premise> {
    type Action = Action<Premise>;

    fn run_action(&mut self, action: Self::Action) {
        match action {
            Action::AmbChoose(a, b) => {
                let true_no_goods = self.premise_nogoods.get(&a);
                let false_no_goods = self.premise_nogoods.get(&b);

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
