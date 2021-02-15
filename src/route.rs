#[derive(Debug)]
pub enum Matched<'a> {
    /* / */
    Home,
    /* /about */
    About,
    /* /links */
    Link,
    /* /post/:slug */
    Post(&'a str),
    /* /amp */
    AMPHome,
    /* /amp/about */
    AMPAbout,
    /* /amp/links */
    AMPLink,
    /* /amp/post/:slug */
    AMPPost(&'a str),
    /* /org/about */
    OrgAbout,
    /* /org/post/:slug */
    OrgPost(&'a str),
    /* /rss | /atom.xml */
    RSS,
    /* /assets/:filename */
    Assets(&'a str),
    /* * */
    NotFound,
}

pub fn match_route<'a>(path: &'a str) -> Matched<'a> {
    if path.starts_with("/assets/") {
        return Matched::Assets(&path["/assets/".len()..]);
    }

    // TODO: possible to collects into [&str; 3]?
    let segs = path.split("/").skip(1).collect::<Vec<_>>();

    match segs.as_slice() {
        [""] => Matched::Home,
        ["about"] => Matched::About,
        ["link"] => Matched::Link,
        ["post", slug] => Matched::Post(slug),
        ["amp"] => Matched::AMPHome,
        ["amp", "about"] => Matched::AMPAbout,
        ["amp", "link"] => Matched::AMPLink,
        ["amp", "post", slug] => Matched::AMPPost(slug),
        ["org", "about"] => Matched::OrgAbout,
        ["org", "post", slug] => Matched::OrgPost(slug),
        ["rss"] | ["atom.xml"] => Matched::RSS,
        ["assets", name] => Matched::Assets(name),
        _ => Matched::NotFound,
    }
}
