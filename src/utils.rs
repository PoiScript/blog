macro_rules! headers(
    { $($key:expr => $value:expr),+ } => {
        {
            let headers = ::web_sys::Headers::new().unwrap();
            $(
                headers.set($key, $value).unwrap();
            )+
            headers
        }
     };
     () => { ::web_sys::Headers::new().unwrap() };
);
