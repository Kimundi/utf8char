extern crate test;
extern crate utf8char;
use test::Bencher;
use utf8char::Utf8Char;
use utf8char::Utf8StrExt;

static SPACE: &'static str = "                                                                                                                                                                                                                                                                                                                           ";
static UTF8SPACE: Utf8Char = Utf8Char { data: [b' ', 0, 0, 0] };
static CHARSPACE: char = ' ';

#[inline(never)]
fn count_leading_utf8_spaces(s: &str) -> uint {
    let mut iter = s.utf8_chars();
    let mut i = 0;
    let c = Some(UTF8SPACE);
    while iter.next() == c {i += 1;} i
}

#[bench]
fn bench_count_leading_utf8_spaces(b: &mut Bencher) {
    b.iter(|| {
        count_leading_utf8_spaces(SPACE);
    })
}

#[inline(never)]
fn count_leading_char_spaces(s: &str) -> uint {
    let mut iter = s.chars();
    let mut i = 0;
    let c = Some(CHARSPACE);
    while iter.next() == c {i += 1;} i
}

#[bench]
fn bench_count_leading_char_spaces(b: &mut Bencher) {
    b.iter(|| {
        count_leading_char_spaces(SPACE);
    })
}
