use std::collections::HashMap;

// a=1&b=2&c&d=&e===&d=7&d=abc

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, &'buf str>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}
