use super::*;
use crate::cards::new_card_from_string;
use crate::cards_h::Card;
use crate::deck::deck_shuffle;
use crate::hands::{hand_compare, hand_sort};

pub const ITERATIONS: i32 = 2000000;

pub(crate) extern "C" fn usage(name: *const i8) -> () {
    unsafe {
        unsafe {
            fprintf(
                __stderrp,
                c"Usage: %s <c1> <c2> <c3> <c4>\n".as_ptr() as *mut i8 as *const i8,
                name,
            )
        };
        unsafe { exit(1) };
    }
}

pub(crate) extern "C" fn __main_inner(argc: i32, argv: *const *mut i8) -> Result<(), i32> {
    if argc != 5 {
        usage(unsafe { *argv.offset(0 as isize) } as *const i8);
    }
    unsafe { srand(unsafe { time(0 as *mut () as *mut TimeT) } as u32) };
    let mut deck: [Card; 48] = unsafe { core::mem::zeroed() };
    let mut cards: [Card; 4] = unsafe { core::mem::zeroed() };
    let mut hand1: [Card; 5] = unsafe { core::mem::zeroed() };
    let mut hand2: [Card; 5] = unsafe { core::mem::zeroed() };
    let mut results: [u64; 3] = [0 as u64, 0 as u64, 0 as u64];
    {
        let mut i: u64 = 0 as u64;
        '__b8: loop {
            if !(i < 4 as u64) {
                break '__b8;
            }
            '__c8: loop {
                cards[i as usize] =
                    new_card_from_string(unsafe { *argv.add(i.wrapping_add(1 as u64) as usize) });
                break '__c8;
            }
            i = i.wrapping_add(1);
        }
    }
    unsafe {
        new_deck(
            &raw mut deck[0 as usize] as *mut Card,
            &raw mut cards[0 as usize] as *mut Card as *const Card,
            4 as u64,
        )
    };
    {
        let mut i: u64 = 0 as u64;
        '__b9: loop {
            if !(i < ITERATIONS as u64) {
                break '__b9;
            }
            '__c9: loop {
                deck_shuffle(&raw mut deck[0 as usize] as *mut Card, 3 as u64, 48 as u64);
                hand1[0 as usize] = cards[0 as usize];
                hand1[1 as usize] = cards[1 as usize];
                hand2[0 as usize] = cards[2 as usize];
                hand2[1 as usize] = cards[3 as usize];
                {
                    let mut j: u64 = 0 as u64;
                    '__b10: loop {
                        if !(j < 3 as u64) {
                            break '__b10;
                        }
                        '__c10: loop {
                            hand1[(2 as u64).wrapping_add(j) as usize] = deck[j as usize];
                            hand2[(2 as u64).wrapping_add(j) as usize] = deck[j as usize];
                            break '__c10;
                        }
                        j = j.wrapping_add(1);
                    }
                }
                hand_sort(&raw mut hand1[0 as usize] as *mut Card);
                hand_sort(&raw mut hand2[0 as usize] as *mut Card);
                let c: i32 = hand_compare(
                    &raw mut hand1[0 as usize] as *mut Card,
                    &raw mut hand2[0 as usize] as *mut Card,
                );
                if c > 0 {
                    {
                        let __p = &mut results[0 as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                } else if c == 0 {
                    {
                        let __p = &mut results[1 as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                } else if c < 0 {
                    {
                        let __p = &mut results[2 as usize];
                        let __t = *__p;
                        *__p = (*__p).wrapping_add(1);
                        __t
                    };
                }
                break '__c9;
            }
            i = i.wrapping_add(1);
        }
    }
    unsafe {
        printf(
            c"WIN: %.2f\tTIE: %.2f\tLOSS: %.2f\n".as_ptr() as *mut i8 as *const i8,
            results[0 as usize] as f64 / ITERATIONS as f64,
            results[1 as usize] as f64 / ITERATIONS as f64,
            results[2 as usize] as f64 / ITERATIONS as f64,
        )
    };
    return Ok(());
}
