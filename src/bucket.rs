use super::*;
use crate::bucket_h::Bucket;
use crate::cards_h::Card;

pub(crate) extern "C" fn bucket_add(bucket: &mut Bucket, card: &Card) -> () {
    if (*bucket).count < 4 as u64 {
        (*bucket).cards[(*bucket).count as usize] = *card;
        {
            let __p = &mut (*bucket).count;
            let __t = *__p;
            *__p = (*__p).wrapping_add(1);
            __t
        };
    }
}
