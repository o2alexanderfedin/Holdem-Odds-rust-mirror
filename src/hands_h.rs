use super::*;

pub const HAND_LENGTH: i32 = 5;

pub(crate) const InvalidHand: u32 = 0;

pub(crate) const HighCard: u32 = 1;

pub(crate) const Pair: u32 = 2;

pub(crate) const TwoPair: u32 = 3;

pub(crate) const ThreeOfAKind: u32 = 4;

pub(crate) const Wheel: u32 = 5;

pub(crate) const Straight: u32 = 6;

pub(crate) const Flush: u32 = 7;

pub(crate) const FullHouse: u32 = 8;

pub(crate) const FourOfAKind: u32 = 9;

pub(crate) const WheelFlush: u32 = 10;

pub(crate) const StraightFlush: u32 = 11;

pub(crate) type HandType = u32;
