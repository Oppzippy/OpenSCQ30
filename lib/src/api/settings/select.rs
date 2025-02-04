#[derive(Clone, Debug)]
pub struct Select {
    pub options: Vec<&'static str>,
}

impl Select {
    pub(crate) fn from_enum<T>(variants: impl IntoIterator<Item = T>) -> Select
    where
        T: Into<&'static str>,
    {
        let options: Vec<&'static str> =
            variants.into_iter().map(|variant| variant.into()).collect();
        Self { options }
    }
}
