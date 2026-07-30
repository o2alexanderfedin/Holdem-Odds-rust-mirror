use super::*;
use crate::cards_h::{Card, Rank, Suit, RANKS_PER_DECK, SUITS_PER_DECK};

pub(crate) extern "C" fn index_of(c: i8, chars: *const i8, n: u64) -> u64 {
    {
        let mut i: u64 = 0 as u64;
        '__b0: loop {
            if !(i < n) {
                break '__b0;
            }
            '__c0: loop {
                if unsafe { *chars.add(i as usize) } as i32 == c as i32 {
                    return i.wrapping_add(1 as u64);
                }
                break '__c0;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0 as u64;
}

pub(crate) static mut invalid_card: Card = unsafe { core::mem::zeroed() };

/// Constructors
pub(crate) extern "C" fn new_card(rank: Rank, suit: Suit) -> Card {
    unsafe {
        let mut card: Card = Card {
            rank: rank,
            suit: suit,
        };
        if unsafe { card_is_valid(&raw mut card as *const Card) } {
            return card;
        } else {
            return invalid_card;
        }
    }
}

pub(crate) extern "C" fn char_to_rank(r: i8) -> Rank {
    return index_of(
        r,
        c"23456789TJQKA".as_ptr() as *mut i8 as *const i8,
        RANKS_PER_DECK as u64,
    ) as Rank;
}

pub(crate) extern "C" fn char_to_suit(s: i8) -> Suit {
    return index_of(
        s,
        c"cdhs".as_ptr() as *mut i8 as *const i8,
        SUITS_PER_DECK as u64,
    ) as Suit;
}

pub(crate) extern "C" fn new_card_from_chars(r: i8, s: i8) -> Card {
    let rank: Rank = char_to_rank(r);
    let suit: Suit = char_to_suit(s);
    return new_card(rank, suit);
}

pub(crate) extern "C" fn new_card_from_string(s: *mut i8) -> Card {
    unsafe {
        if unsafe { strlen(s as *const i8) } != 2 as u64 {
            return invalid_card;
        } else {
            return new_card_from_chars(unsafe { *s.offset(0 as isize) }, unsafe {
                *s.offset(1 as isize)
            });
        }
    }
}

pub(crate) extern "C" fn card_compare(a: &Card, b: &Card) -> i32 {
    return (*a).rank.wrapping_sub((*b).rank) as i32;
}

pub(crate) extern "C" fn card_equal(a: &Card, b: &Card) -> bool {
    return (*a).rank == (*b).rank && (*a).suit == (*b).suit;
}

pub(crate) extern "C" fn card_to_string(out: *mut i8, c: *const Card) -> () {
    if !unsafe { card_is_valid(c) } as i32 != 0 {
        unsafe { *out.offset(0 as isize) = '-' as i32 as i8 };
        unsafe { *out.offset(1 as isize) = '-' as i32 as i8 };
    } else {
        unsafe {
            *out.offset(0 as isize) = unsafe {
                *(c"23456789TJQKA".as_ptr() as *mut i8)
                    .offset((unsafe { (*c).rank } as i32 - 1) as isize)
            }
        };
        unsafe {
            *out.offset(1 as isize) = unsafe {
                *(c"cdhs".as_ptr() as *mut i8).offset((unsafe { (*c).suit } as i32 - 1) as isize)
            }
        };
    }
    unsafe { *out.offset(2 as isize) = '\u{0}' as i32 as i8 };
}

pub(crate) extern "C" fn card_swap(a: &mut Card, b: &mut Card) -> () {
    let t: Card = *a;
    *a = *b;
    *b = t;
}
