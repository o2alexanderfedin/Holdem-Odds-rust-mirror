#![allow(unused_imports, dead_code)]

mod bucket;
mod bucket_h;
mod cards;
mod cards_h;
mod deck;
mod hands;
mod hands_h;
mod holdemodds;
use crate::cards_h::Card;
use crate::holdemodds::__main_inner;

pub(crate) type DarwinTimeT = i64;

pub(crate) type TimeT = DarwinTimeT;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub(crate) struct SFILE {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type FILE = SFILE;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn card_is_valid(_: *const Card) -> bool;
    fn strlen(__s: *const i8) -> u64;
    fn rand() -> i32;
    fn new_deck(_: *mut Card, _: *const Card, _: u64) -> ();
    fn hand_is_wheel(_: *const Card) -> bool;
    fn __builtin_object_size(_: *const (), _: i32) -> u64;
    fn __builtin___memcpy_chk(_: *mut (), _: *const (), _: u64, _: u64) -> *mut ();
    fn fprintf(_: *mut FILE, _: *const i8, ...) -> i32;
    fn exit(_: i32) -> ();
    fn time(_: *mut TimeT) -> TimeT;
    fn srand(_: u32) -> ();
    fn printf(_: *const i8, ...) -> i32;
    static mut __stderrp: *mut FILE;
}
