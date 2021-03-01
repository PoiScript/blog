use chrono::{DateTime, NaiveDate, Utc};
use web_sys::*;

macro_rules! headers(
    { $($key:expr => $value:expr),+ } => {
        {
            let headers = ::web_sys::Headers::new().unwrap();
            headers.set("x-content-type-options", "nosniff").unwrap();
            $(
                headers.set($key, $value).unwrap();
            )+
            headers
        }
     };
     () => { ::web_sys::Headers::new().unwrap() };
);

pub fn html_response(body: &str) -> Response {
    Response::new_with_opt_str_and_init(
        Some(body),
        ResponseInit::new().status(200).headers(
            &headers!(
                "content-type" => "text/html; charset=utf-8",
                "cache-control" => "no-cache"
            )
            .into(),
        ),
    )
    .unwrap()
}

pub fn redirect_404_response() -> Response {
    Response::new_with_opt_str_and_init(
        Some("Redirecting to /404"),
        ResponseInit::new().status(302).headers(
            &headers!(
                "location" => "/404",
                "content-type" => "text/plain; charset=utf-8"
            )
            .into(),
        ),
    )
    .unwrap()
}

pub fn to_datetime(date: NaiveDate) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc)
}
