pub trait Translate {
    fn translate(&self) -> String;
}

impl<T> Translate for &T
where
    T: Translate,
{
    fn translate(&self) -> String {
        T::translate(self)
    }
}
