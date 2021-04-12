#![allow(clippy::unreadable_literal)]

use orgize::{
    export::{DefaultHtmlHandler, HtmlEscape as Escape, HtmlHandler},
    Element,
};
use std::{
    io::{Error, Result, Write},
    ops::Range,
};

#[derive(Default)]
pub struct SolomonBaseHandler {
    default: DefaultHtmlHandler,
    last_char: Option<char>,
}

impl HtmlHandler<Error> for SolomonBaseHandler {
    fn start<W: Write>(&mut self, mut w: W, element: &Element) -> Result<()> {
        match element {
            Element::Document { .. } => (),
            Element::Paragraph { .. } => {
                self.last_char = None;
                write!(w, "<p>")?;
            }
            Element::Link(link) => {
                let text = link.desc.as_ref().unwrap_or(&link.path);
                if should_insert_space(self.last_char, text.chars().next()) {
                    write!(w, " ")?;
                }
                self.last_char = None;

                write!(w, "<a href=\"{}\">", Escape(&link.path))?;

                for line in text.lines() {
                    let text = line.trim();
                    let first_char = text.chars().next();
                    if should_insert_space(self.last_char, first_char) {
                        write!(w, " ")?;
                    }
                    self.last_char = text.chars().last();

                    write!(w, "{}", Escape(text))?;
                }

                write!(w, "</a>")?;
            }
            Element::Text { value } => {
                for line in value.lines() {
                    let text = line.trim();
                    let first_char = text.chars().next();
                    if should_insert_space(self.last_char, first_char) {
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

            // code highlighting
            Element::InlineSrc(inline_src) => write!(
                w,
                r#"<code class="lang-{}">{}</code>"#,
                &inline_src.lang, &inline_src.body,
            )?,
            Element::SourceBlock(block) => write!(
                w,
                r#"<pre><code class="lang-{}">{}</code></pre>"#,
                &block.language, &block.contents
            )?,
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

fn should_insert_space(c1: Option<char>, c2: Option<char>) -> bool {
    const CJK_CHARACTERS: Range<u32> = 0x4E00..0x9FFF;

    const CJK_PUNCTUATIONS: [char; 14] = [
        '。', '？', '，', '、', '；', '：', '“', '”', '「', '」', '（', '）', '《', '》',
    ];

    if let (Some(c1), Some(c2)) = (c1, c2) {
        (c1.is_ascii_graphic() && c2.is_ascii_graphic())
            || (c1.is_ascii_graphic()
                && CJK_CHARACTERS.contains(&(c2 as u32))
                && !CJK_PUNCTUATIONS.contains(&c2))
            || (c2.is_ascii_graphic()
                && CJK_CHARACTERS.contains(&(c1 as u32))
                && !CJK_PUNCTUATIONS.contains(&c1))
    } else {
        false
    }
}
