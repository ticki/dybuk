extern crate term_painter;
extern crate regex;

mod parse;
mod print;
mod wrap;





fn main() {
    let op = parse::MessageIter::new();

    for i in op {
        for l in i {
            l.print();
        }
    }
}
