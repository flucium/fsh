use std::borrow::Cow;

pub fn parse(source: impl Into<String>) -> Cow<'static, str> {
    source.into().into()
}
