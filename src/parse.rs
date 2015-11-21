use std::io::stdin;
use std::io::prelude::*;
use std::iter::once;
use regex::Regex;

pub struct MessageIter {
    buf: String,
}

pub enum Message {
    Header(String, String),
    Warning(String),
    Note(String),
    Error(String),
    Help(String),
    FollowUp(String),
    Source(String, String),
    Marker(String),
    NewLine,
    Wat,
}

impl Iterator for MessageIter {
    type Item = Vec<Message>;

    fn next(&mut self) -> Option<Vec<Message>> {
        use self::Message::*;

        let mut res = Vec::new();
        let mut file = String::new();
        let mut stop = false;
        let si = stdin();
        let stdin = si.lock().lines().map(|x| x.expect("Stdin failed"));

        for l in once(self.buf.clone()).chain(stdin) {
            let re_header = Regex::new(r"([0-9A-Za-z_\.\\/]+):(\d+):\d+: .*(warning: |note: |error: |help: )(.*)").unwrap();
            let re_source = Regex::new(r"(\d+) (.*)").unwrap();
            if re_header.is_match(&l) {
                if !stop {
                    stop = true;
                } else {
                    self.buf = l.to_string();
                    return Some(res);
                }

                res.push(NewLine);

                let caps = re_header.captures(&l).unwrap();
                file = caps.at(1).unwrap_or("?").to_string();

                res.push(Header(file.clone(), caps.at(2).unwrap_or("?").to_string()));

                let msg = caps.at(4).unwrap_or("?").to_string();

                // Warning, header or note?
                match caps.at(3).unwrap_or("?") {
                    "warning: " => res.push(Warning(msg)),
                    "note: " =>    res.push(Note(msg)),
                    "error: " =>   res.push(Error(msg)),
                    "help: " =>    res.push(Help(msg)),
                    _ =>           res.push(Wat),
                }
            } else if l.len() > file.len() && re_source.is_match(&l[file.len()..]) {
                let caps = re_source.captures(&l).unwrap();

                res.push(Source(caps.at(1).unwrap_or("?").to_string(), caps.at(2).unwrap_or("????").to_string()));
            } else if l.chars().next() == Some(' ') && l.contains("^") {

                let offset = if let Some(&Source(ref line, _)) = res.last() {
                    file.len() + line.len().saturating_sub(6)
                } else {
                    0
                };

                if offset < l.len() {
                    res.push(Marker(l[offset..].to_string()));
                }
            } else {
                res.push(FollowUp(l.to_string()));
            }
        }
        None
    }
}

impl MessageIter {
    pub fn new() -> Self {
        MessageIter {
            buf: String::new(),
        }
    }
}

