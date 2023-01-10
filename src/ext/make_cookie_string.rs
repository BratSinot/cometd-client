use cookie::{Cookie, CookieJar};

pub(crate) trait CookieJarExt {
    fn make_string(&self) -> Box<str>;
}

impl CookieJarExt for CookieJar {
    fn make_string(&self) -> Box<str> {
        self.iter()
            .map(Cookie::name_value)
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Box<[_]>>()
            .join("; ")
            .into_boxed_str()
    }
}
