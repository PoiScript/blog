#![allow(clippy::unreadable_literal)]

use chrono::{DateTime, NaiveDateTime, Utc};
use orgize::{
    export::{DefaultHtmlHandler, HtmlEscape as Escape, HtmlHandler},
    Element,
};
use std::io::{Error, Result, Write};

#[derive(Default)]
struct SolomonBaseHandler {
    default: DefaultHtmlHandler,
    last_char: Option<char>,
}

impl HtmlHandler<Error> for SolomonBaseHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<()> {
        match element {
            Element::Document { .. } => (),
            Element::Macros(macros) => match &macros.name as &str {
                "age-days" => {
                    let date =
                        DateTime::from_utc(NaiveDateTime::from_timestamp(1382071200, 0), Utc);

                    write!(w, " {} ", (Utc::now() - date).num_days())?;
                }
                _ => (),
            },
            Element::Paragraph { .. } => {
                self.last_char = None;
                write!(w, "<p>")?;
            }
            Element::Link(link) => {
                let text = link.desc.as_ref().unwrap_or(&link.path);
                if should_insert_space(self.last_char, text.chars().next()) {
                    write!(w, " ")?;
                }
                self.last_char = text.chars().last();
                write!(w, "<a href=\"{}\">{}</a>", Escape(&link.path), Escape(text))?;
            }
            Element::Text { value } => {
                for line in value.lines() {
                    let text = line.trim();
                    if should_insert_space(self.last_char, text.chars().next()) {
                        write!(w, " ")?;
                    }
                    self.last_char = text.chars().last();
                    write!(w, "{}", Escape(text))?;
                }
            }
            Element::Verbatim { value } | Element::Code { value } => {
                let text = value.trim();
                if should_insert_space(self.last_char, text.chars().next()) {
                    write!(w, " ")?;
                }
                self.last_char = text.chars().last();
                write!(w, "<code>{}</code>", Escape(text))?;
            }
            _ => self.default.start(w, element)?,
        }
        Ok(())
    }

    fn end<W: Write>(&mut self, w: W, element: &Element) -> Result<()> {
        match element {
            Element::Document { .. } => (),
            _ => self.default.end(w, element)?,
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct SolomonHtmlHandler(SolomonBaseHandler);

impl HtmlHandler<Error> for SolomonHtmlHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<()> {
        match element {
            Element::Link(link) if link.path.starts_with("file:") => {
                let path = &link.path[5..];

                write!(
                    w,
                    "<div class=\"image-container\">\
                     <div class=\"image-wrapper\">\
                     <img src=\"{}\" loading=\"lazy\"/>\
                     </div></div>",
                    path
                )?;

                Ok(())
            }
            _ => self.0.start(w, element),
        }
    }

    fn end<W: Write>(&mut self, w: W, element: &Element) -> Result<()> {
        self.0.end(w, element)
    }
}

fn should_insert_space(c1: Option<char>, c2: Option<char>) -> bool {
    const PUNCTUATIONS: [char; 14] = [
        '。', '？', '，', '、', '；', '：', '“', '”', '「', '」', '（', '）', '《', '》',
    ];

    if let (Some(c1), Some(c2)) = (c1, c2) {
        (c1.is_ascii_graphic() && c2.is_ascii_graphic())
            || (c1.is_ascii_graphic()
                && 0x4E00 < (c2 as u32)
                && (c2 as u32) < 0x9FFF
                && !PUNCTUATIONS.contains(&c2))
            || (c2.is_ascii_graphic()
                && 0x4E00 < (c1 as u32)
                && (c1 as u32) < 0x9FFF
                && !PUNCTUATIONS.contains(&c1))
    } else {
        false
    }
}
