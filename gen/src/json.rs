use json::{object, JsonValue};
use std::fs;

use crate::{entry::Entry, error::Result, handlers::SolomonHtmlHandler};

pub fn write(entries: &[Entry]) -> Result<()> {
    match entries.len() {
        0 => {}
        1 => {
            write_internal(&entries[0], None, None)?;
        }
        len @ _ => {
            write_internal(&entries[0], None, Some(&entries[1]))?;

            for windows in entries.windows(3) {
                write_internal(&windows[1], Some(&windows[0]), Some(&windows[2]))?;
            }

            write_internal(&entries[len - 1], Some(&entries[len - 2]), None)?;
        }
    }

    Ok(())
}

fn write_internal(curr: &Entry, prev: Option<&Entry>, next: Option<&Entry>) -> Result<()> {
    let mut vec = Vec::new();
    let mut handler = SolomonHtmlHandler::default();
    curr.org.write_html_custom(&mut vec, &mut handler)?;

    let html = String::from_utf8(vec)?;

    let mut obj = object! {
        "title" => &*curr.title,
        "slug" => &*curr.slug,
        "tags" => &*curr.tags,
        "published" => curr.published.to_rfc3339(),
        "updated" => curr.updated.map(|d| d.to_rfc3339()),
        "html" => html,
    };

    if let Some(prev) = prev {
        obj.insert(
            "prev",
            object! {
                "title" => &*prev.title,
                "slug" => &*prev.slug,
            },
        )?;
    }

    if let Some(next) = next {
        obj.insert(
            "next",
            object! {
                "title" => &*next.title,
                "slug" => &*next.slug,
            },
        )?;
    }

    fs::write(format!("assets/post/{}.json", curr.slug), obj.to_string())?;

    Ok(())
}

pub fn write_posts(entries: &[Entry]) -> Result<()> {
    let entries = JsonValue::Array(
        entries
            .iter()
            .map(|entry| {
                object! {
                    "title" => &*entry.title,
                    "slug" => &*entry.slug,
                    "tags" => &*entry.tags,
                    "published" => entry.published.to_rfc3339(),
                }
            })
            .collect::<Vec<_>>(),
    );

    fs::write("assets/posts.json", entries.to_string())?;

    Ok(())
}
