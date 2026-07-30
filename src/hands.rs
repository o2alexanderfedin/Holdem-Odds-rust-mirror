use super::*;
use crate::bucket::bucket_add;
use crate::bucket_h::Bucket;
use crate::cards::card_compare;
use crate::cards_h::{Card, RANKS_PER_DECK};
use crate::hands_h::{
    Flush, FourOfAKind, FullHouse, HandType, HighCard, Pair, Straight, StraightFlush, ThreeOfAKind,
    TwoPair, Wheel, WheelFlush, HAND_LENGTH,
};

extern "C" fn hand_is_flush(cards: *const Card) -> bool {
    {
        let mut i: u64 = 1 as u64;
        '__b2: loop {
            if !(i < HAND_LENGTH as u64) {
                break '__b2;
            }
            '__c2: loop {
                if unsafe { (*cards.add(i as usize)).suit }
                    != unsafe { (*cards.offset(0 as isize)).suit }
                {
                    return 0;
                }
                break '__c2;
            }
            i = i.wrapping_add(1);
        }
    }
    return 1;
}

extern "C" fn hand_is_straight(cards: *const Card) -> bool {
    return unsafe { (*cards.offset(0 as isize)).rank }
        == unsafe { (*cards.offset(1 as isize)).rank.wrapping_add(1 as u32) }
        && unsafe { (*cards.offset(0 as isize)).rank }
            == unsafe { (*cards.offset(4 as isize)).rank.wrapping_add(4 as u32) };
}

extern "C" fn hand_is_four_of_a_kind(cards: *const Card) -> bool {
    return unsafe { (*cards.offset(0 as isize)).rank }
        == unsafe { (*cards.offset(3 as isize)).rank };
}

extern "C" fn hand_is_full_house(cards: *const Card) -> bool {
    return unsafe { (*cards.offset(0 as isize)).rank }
        == unsafe { (*cards.offset(2 as isize)).rank }
        && unsafe { (*cards.offset(3 as isize)).rank }
            == unsafe { (*cards.offset(4 as isize)).rank };
}

extern "C" fn hand_is_three_of_a_kind(cards: *const Card) -> bool {
    return unsafe { (*cards.offset(0 as isize)).rank }
        == unsafe { (*cards.offset(2 as isize)).rank };
}

extern "C" fn hand_is_two_pair(cards: *const Card) -> bool {
    return unsafe { (*cards.offset(2 as isize)).rank }
        == unsafe { (*cards.offset(3 as isize)).rank };
}

extern "C" fn hand_is_pair(cards: *const Card) -> bool {
    return unsafe { (*cards.offset(0 as isize)).rank }
        == unsafe { (*cards.offset(1 as isize)).rank };
}

pub(crate) extern "C" fn hand_classify(cards: *mut Card) -> HandType {
    if hand_is_flush(cards as *const Card) {
        if hand_is_straight(cards as *const Card) {
            return StraightFlush;
        }
        if unsafe { hand_is_wheel(cards as *const Card) } {
            return WheelFlush;
        }
        return Flush;
    } else {
        if hand_is_four_of_a_kind(cards as *const Card) {
            return FourOfAKind;
        }
        if hand_is_full_house(cards as *const Card) {
            return FullHouse;
        }
        if hand_is_straight(cards as *const Card) {
            return Straight;
        }
        if unsafe { hand_is_wheel(cards as *const Card) } {
            return Wheel;
        }
        if hand_is_three_of_a_kind(cards as *const Card) {
            return ThreeOfAKind;
        }
        if hand_is_two_pair(cards as *const Card) {
            return TwoPair;
        }
        if hand_is_pair(cards as *const Card) {
            return Pair;
        }
        return HighCard;
    }
}

pub(crate) extern "C" fn hand_compare(hand1: *mut Card, hand2: *mut Card) -> i32 {
    let hand1_type: HandType = hand_classify(hand1);
    let hand2_type: HandType = hand_classify(hand2);
    let diff: i32 = hand1_type.wrapping_sub(hand2_type) as i32;
    if diff != 0 {
        return diff;
    }
    {
        let mut i: u64 = 0 as u64;
        '__b3: loop {
            if !(i < HAND_LENGTH as u64) {
                break '__b3;
            }
            '__c3: loop {
                let cmp: i32 = card_compare(unsafe { &*hand1.add(i as usize) }, unsafe {
                    &*hand2.add(i as usize)
                });
                if cmp != 0 {
                    return cmp;
                }
                break '__c3;
            }
            i = i.wrapping_add(1);
        }
    }
    return 0;
}

pub(crate) extern "C" fn hand_sort(hand: *mut Card) -> () {
    let mut buckets: [Bucket; 13] = unsafe { core::mem::zeroed() };
    {
        let mut i: u64 = 0 as u64;
        '__b4: loop {
            if !(i < HAND_LENGTH as u64) {
                break '__b4;
            }
            '__c4: loop {
                let bucket_index: u64 =
                    unsafe { (*hand.add(i as usize)).rank.wrapping_sub(1 as u32) } as u64;
                bucket_add(&mut buckets[bucket_index as usize], unsafe {
                    &*hand.add(i as usize)
                });
                break '__c4;
            }
            i = i.wrapping_add(1);
        }
    }
    let mut index: u64 = 0 as u64;
    {
        let mut count: u64 = 4 as u64;
        '__b5: loop {
            if !(count > 0 as u64) {
                break '__b5;
            }
            '__c5: loop {
                {
                    let mut j: u64 = (RANKS_PER_DECK - 1) as u64;
                    '__b6: loop {
                        if !(j != -1i32 as u64) {
                            break '__b6;
                        }
                        '__c6: loop {
                            if buckets[j as usize].count == count {
                                unsafe {
                                    __builtin___memcpy_chk(
                                        unsafe { &raw mut *hand.add(index as usize) } as *mut (),
                                        &raw mut buckets[j as usize].cards as *const (),
                                        core::mem::size_of::<Card>() as u64 * count,
                                        unsafe {
                                            __builtin_object_size(
                                                unsafe { &raw mut *hand.add(index as usize) }
                                                    as *const (),
                                                0,
                                            )
                                        },
                                    )
                                };
                                index = index.wrapping_add(count);
                            }
                            break '__c6;
                        }
                        j = j.wrapping_sub(1);
                    }
                }
                break '__c5;
            }
            count = count.wrapping_sub(1);
        }
    }
}

pub(crate) extern "C" fn hand_is_valid(cards: *const Card) -> bool {
    {
        let mut i: u64 = 0 as u64;
        '__b7: loop {
            if !(i < HAND_LENGTH as u64) {
                break '__b7;
            }
            '__c7: loop {
                if !unsafe { card_is_valid(unsafe { &*cards.add(i as usize) }) } as i32 != 0 {
                    return 0;
                }
                break '__c7;
            }
            i = i.wrapping_add(1);
        }
    }
    return 1;
}

extern "C" fn hand_is_straight_flush(cards: *const Card) -> bool {
    return hand_is_straight(cards) && hand_is_flush(cards);
}

extern "C" fn hand_is_wheel_flush(cards: *const Card) -> bool {
    return unsafe { hand_is_wheel(cards) } && hand_is_flush(cards);
}

extern "C" fn hand_is_high_card(cards: *const Card) -> bool {
    {
        let _ = cards;
    };
    return 1;
}
