

pub type TruthManagementStore<T, U> = Content<TruthManagementStoreImpl<T, U>>;

#[derive(Clone)]
pub struct TruthManagementStoreImpl<T, U: Premise> {
    context: Rc<TruthManagementContext<U>>,
    supports: Vec<>
}
