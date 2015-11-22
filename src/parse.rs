use std::io::stdin;
use std::io::prelude::*;
use std::iter::once;
use regex::Regex;

pub struct MessageIter {
    buf: String,
    terminated: bool,
    pub errors: u16,
    pub warnings: u16,
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
    Aborting,
}

impl<'a> Iterator for &'a mut MessageIter {
    type Item = Vec<Message>;

    fn next(&mut self) -> Option<Vec<Message>> {
        use self::Message::*;

        if self.terminated {
            return None;
        }

        let mut res = Vec::new();
        let mut file = String::new();
        let mut stop = false;
        let si = stdin();
        let stdin = si.lock().lines().map(|x| x.expect("Stdin failed"));

        for l in once(self.buf.clone()).chain(stdin) {
            let re_header = Regex::new(r"([0-9A-Za-z_\.\\/>< ]+):(\d+):\d+: .*(warning: |note: |error: |help: )(.*)").unwrap();
            let re_source = Regex::new(r"(\d+) (.*)").unwrap();
            if re_header.is_match(&l) {

                res.push(NewLine);

                let caps = re_header.captures(&l).unwrap();
                file = caps.at(1).unwrap_or("?").to_string();

                res.push(Header(file.clone(), caps.at(2).unwrap_or("?").to_string()));

                let msg = caps.at(4).unwrap_or("?").to_string();

                // Warning, header or note?
                match caps.at(3).unwrap_or("?") {
                    "warning: " => {
                        self.warnings += 1;
                        res.push(Warning(msg));
                    },
                    "note: " =>    res.push(Note(msg)),
                    "error: " => {
                        self.errors += 1;
                        res.push(Error(msg));
                    },
                    "help: " =>    res.push(Help(msg)),
                    _ =>           res.push(Wat),
                }

                if !stop {
                    stop = true;
                } else {
                    self.buf = l.to_string();
                    return Some(res);
                }
            } else if l.len() > file.len() && re_source.is_match(&l[file.len()..]) && is_not_cmd(&l) {
                let caps = re_source.captures(&l).unwrap();

                res.push(Source(caps.at(1).unwrap_or("?").to_string(), caps.at(2).unwrap_or("????").to_string()));
            } else if l.starts_with(' ') && l.contains("^") {

                let offset = file.len() - 4; //+ 5 - 5;

                if offset < l.len() {
                    res.push(Marker(l[offset..].to_string()));
                }
            } else if l.contains("Aborting due to previous") || l.contains("Build failed") || l.contains("Could not compile") {
                res.push(Aborting);
                stop = true;
                self.terminated = true;
            } else if l.contains("Compilining ") || l.contains("file:///home/") || l.is_empty() {
                // todo
            } else if is_not_cmd(&l){
                res.push(FollowUp(l.to_string()));
            }
        }

        self.terminated = true;
        if stop {
            Some(res)
        } else {
            None
        }
    }
}

fn is_not_cmd(l: &str) -> bool {
    l.len() < 30 || !(l.starts_with("rustc ") || l.starts_with("cargo ") || l.starts_with("make ") || l.contains(" --") || l.contains(" --target=") || l.contains(" -C ") || l.contains(" -L ") || l.contains(" -A ") || l.contains(" -Z ") || l.contains(" -o ") || l.starts_with("sed ") || l.starts_with("mkdir ") || l.starts_with("cd "))
}

impl MessageIter {
    pub fn new() -> Self {
        MessageIter {
            buf: String::new(),
            terminated: false,
            errors: 0,
            warnings: 0,
        }
    }
}

