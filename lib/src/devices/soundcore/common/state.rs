pub trait Update<T> {
    fn update(&mut self, partial: T);
}

impl<T, T2> Update<T> for T2
where
    T2: From<T>,
{
    fn update(&mut self, partial: T) {
        *self = Self::from(partial);
    }
}
