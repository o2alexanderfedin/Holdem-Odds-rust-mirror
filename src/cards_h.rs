use super::*;

pub(crate) const InvalidSuit: u32 = 0;

pub(crate) const Club: u32 = 1;

pub(crate) const Diamond: u32 = 2;

pub(crate) const Heart: u32 = 3;

pub(crate) const Spade: u32 = 4;

pub(crate) type Suit = u32;

pub(crate) const InvalidRank: u32 = 0;

pub(crate) const Deuce: u32 = 1;

pub(crate) const Trey: u32 = 2;

pub(crate) const Four: u32 = 3;

pub(crate) const Five: u32 = 4;

pub(crate) const Six: u32 = 5;

pub(crate) const Seven: u32 = 6;

pub(crate) const Eight: u32 = 7;

pub(crate) const Nine: u32 = 8;

pub(crate) const Ten: u32 = 9;

pub(crate) const Jack: u32 = 10;

pub(crate) const Queen: u32 = 11;

pub(crate) const King: u32 = 12;

pub(crate) const Ace: u32 = 13;

pub(crate) type Rank = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct Card {
    pub(crate) rank: Rank,
    pub(crate) suit: Suit,
}

pub const RANKS_PER_DECK: i32 = 13;

pub const SUITS_PER_DECK: i32 = 4;
