use crate::content::Content;
use crate::util::CellID;
use crate::context_tms::{TruthManagementContext};
use std::collections::HashSet;
use std::rc::Rc;

pub type TruthManagementStore<T, Premise> = Content<TruthManagementStoreImpl<T, Premise>>;

#[derive(Clone)]
pub struct Support<T, Premise> {
    value: T,

    hypotheticals_true: Vec<CellID>,
    hypotheticals_false: Vec<CellID>,
    premises: Vec<Premise>
}


impl<T: Clone, Premise: Ord> Support<T, Premise> {
    pub fn new(value: &T, premises: &[Premise]) -> Self {
        Self {
            value: (*value).clone(),
            premises: premises.iter().cloned().collect().sort()
        }
    }
}

impl<T: PartialEq, Premise: Ord> PartialEq for Support<T, Premise> {
    fn eq(&self, other: &Self) -> bool {
        if self.value != other.value {
            return false;
        }

        if self.premises.len() != other.premises.len() {
            return false;
        }

        if self.premises.len() != other.premises.len() {
            return false;
        }

        //check sorted

        return true;
    }
}

pub struct TruthManagementStoreImpl<T, Premise> {
    context: Rc<TruthManagementContext<Premise>>,
    supports: HashSet<Support<T, Premise>>
}

impl<T: PartialEq, Premise: Ord> PartialEq for TruthManagementStore<T, Premise> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nothing, Self::Nothing) => true,
            (Self::Value(a), Self::Value(b)) => { a == b }
            _ => false,
        }
    }
}

impl<T: Clone + PartialEq + Eq, Premise> TruthManagementStore<T, Premise> {
    pub fn new(
        tms: &Rc<TruthManagementContext<Premise>>, 
        in_supports: &[(T, &[Premise])]
    ) -> Self {
        let mut supports : HashSet<Support<T>> = HashSet::new();

        for (value, premises) in in_supports {
            let support = Support::new(value, premises);

            supports.insert(support);
        }

        Self::Value(TruthManagementStoreImpl {
            context: Rc::clone(tms),
            supports: supports
        })
    }
}
