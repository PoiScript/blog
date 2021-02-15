use chrono::NaiveDate;
use orgize::{Element, Event, Org};

pub struct UpNext {
    pub title: String,
    pub slug: String,
}

pub struct Post {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: NaiveDate,
    pub updated: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub prev: Option<UpNext>,
    pub next: Option<UpNext>,
}

fn parse_timestamp(value: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(value, "[%Y-%m-%d %a]").ok()
}

impl Post {
    pub fn from(content: &str) -> Option<Post> {
        let org = Org::parse(content);
        let (mut title, mut published, mut updated, mut slug, mut tags) =
            (None, None, None, None, None);

        for event in org.iter() {
            if let Event::Start(Element::Keyword(keyword)) = event {
                match &*keyword.key {
                    "TITLE" => title = Some(keyword.value.to_string()),
                    "PUBLISHED" => {
                        published = Some(parse_timestamp(&keyword.value)?);
                    }
                    "UPDATED" => {
                        updated = Some(parse_timestamp(&keyword.value)?);
                    }
                    "TAGS" => {
                        tags = Some(
                            keyword
                                .value
                                .split_whitespace()
                                .map(|s| s.to_string())
                                .collect(),
                        )
                    }
                    "SLUG" => slug = Some(keyword.value.to_string()),
                    _ => (),
                }
            }
        }

        Some(Post {
            published: published.expect("Missing keyword PUBLISHED"),
            updated,
            content: content.into(),
            title: title.expect("Missing keyword TITLE"),
            slug: slug.expect("Missing keyword SLUG"),
            tags: tags.expect("Missing keyword TAGS"),
            next: None,
            prev: None,
        })
    }
}
