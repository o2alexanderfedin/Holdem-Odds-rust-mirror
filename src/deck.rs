use super::*;
use crate::cards::card_swap;
use crate::cards_h::Card;

pub(crate) extern "C" fn deck_shuffle(deck: *mut Card, n: u64, deck_size: u64) -> () {
    {
        let mut i: u64 = 0 as u64;
        '__b1: loop {
            if !(i < n) {
                break '__b1;
            }
            '__c1: loop {
                let j: u64 = unsafe { rand() } as u64 % deck_size;
                card_swap(unsafe { &mut *deck.add(i as usize) }, unsafe {
                    &mut *deck.add(j as usize)
                });
                break '__c1;
            }
            {
                let __p = &mut i;
                *__p = (*__p).wrapping_add(1);
                *__p
            };
        }
    }
}
