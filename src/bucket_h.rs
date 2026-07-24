use super::*;
use crate::cards_h::Card;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Bucket {
    pub(crate) cards: [Card; 4],
    pub(crate) count: u64,
}
