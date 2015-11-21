use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use parse::Message;
use parse::Message::*;
use wrap::wrap_msg;

impl Message {

    pub fn print(self) {

        match self {
            Header(ref file, ref line) =>  println!("+---- {} : {} ----+", Blue.bold().paint(file), Blue.paint(line)),
            Warning(warn) =>               println!("      =====>  {}{}", Yellow.bold().paint("warning: "), Bold.paint(&wrap_msg(warn, 9))),
            Note(note) =>                  println!("      =====>  {}{}", Green.bold().paint("note: "), Bold.paint(&wrap_msg(note, 6))),
            Error(err) =>                  println!("      =====>  {}{}", Red.bold().paint("error: "), Bold.paint(&wrap_msg(err, 7))),
            Help(err) =>                   println!("      =====>  {}{}", Blue.bold().paint("help: "), Bold.paint(&wrap_msg(err, 6))),
            FollowUp(msg) =>               println!("           >  {}", Bold.paint(msg)),
            Source(line, code) =>          println!(" {}  {}", Magenta.paint(format!("{} |>", line)), code),
            Marker(ref mrk) =>             println!("{}", Yellow.paint(mrk)),
            NewLine =>                     println!("\n"),
            Wat =>                         println!("Dafuq?"),
        }
    }
}
