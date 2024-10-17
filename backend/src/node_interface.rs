use crate::types::*;
use tch::Tensor;

pub(crate) trait NodeInterface<T> {
    fn new() -> Self;
    fn reverse_data(&mut self);
    fn pop_last_data_and_index(&mut self) -> Option<(Tensor, ChildType<T>)>;

    fn push_back(&mut self, index: Tensor, datum: ChildType<T>, loc: usize);

    fn get_loc(&self, index: &Tensor) -> usize;

    fn get_midpoint_idx(&self) -> usize;

    fn get_index_len(&self) -> usize;

    // fn push(&mut self, index: Tensor, datum: ChildType<T>);

    // fn push_last_element(&mut self);

    // fn pop_usize_max(&mut self);

    // fn move_data_to(&mut self, other: Box<Self>);

    fn is_empty(&self) -> bool {
        self.get_index_len() == 0
    }
}
