
use std::str::Chars;
use crate::utils::*;

pub(crate) struct Reader<'a> {
    iter: Chars<'a>,
    pushback: Option<char>,
}

impl<'a> Reader<'a> {
    pub fn new(chars: &'a str) -> Reader {
        Reader {
            iter: chars.chars(),
            pushback: None,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.pushback.take().or_else(|| self.iter.next())
    }

    pub fn ignore_if_next(&mut self, ignored_ch: char) -> bool {
        if let Some(ch) = self.next_char() {
            if ch == ignored_ch {
                return true;
            }
            self.push_back(ch);
        }
        false
    }

    pub fn push_back(&mut self, ch: char) {
        self.pushback = Some(ch);
    }


    pub fn get_until(&mut self, stop_on_ws: bool, end_chars: &[char]) -> String {
        let mut result = "".to_owned();
        while let Some(ch) = self.next_char() {
            if (stop_on_ws && is_html_whitespace(ch)) || end_chars.contains(&ch) {
                self.push_back(ch);
                break;
            } else {
                result.push(ch);
            }
        }

        result
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(ch) = self.next_char() {
            if !is_html_whitespace(ch) {
                self.push_back(ch);
                break;
            }
        }
    }

    pub fn read_text(&mut self, end: &[char]) -> String {
        let mut content = String::new();

        while let Some(ch) = self.next_char() {
            if end.contains(&ch) {
                self.push_back(ch);
                break;
            } else if ch == '&' {
                let start = content.len();
                let mut finished = false;

                content.push(ch);

                while let Some(ch) = self.next_char() {
                    if end.contains(&ch) || ch == ' ' {
                        self.push_back(ch);
                        break;
                    } else if ch == ';' {
                        finished = true;
                        break;
                    } else {
                        content.push(ch);
                    }
                }

                if finished {
                    if let Some(ch) = decode_entity(&content[start + 1..]) {
                        content.truncate(start);
                        content.push(ch);
                    } else {
                        content.push(';');
                    }
                }
            } else {
                content.push(ch);
            }
        }

        content
    }

    pub fn read_raw(&mut self, end: &str) -> String {
        let mut content = String::new();

        while let Some(ch) = self.next_char() {
            content.push(ch);
            if content.ends_with(end) {
                let len = content.len();
                content.truncate(len - end.len());
                break;
            }
        }

        content
    }

    pub fn read_raw_count(&mut self, count: usize) -> String {
        let mut content = String::new();

        for _ in 0..count {
            match self.next_char() {
                Some(ch) => content.push(ch),
                None => break,
            }
        }

        content
    }
}
