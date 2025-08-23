pub trait Has<T> {
    fn get(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

pub trait MaybeHas<T> {
    fn maybe_get(&self) -> Option<&T>;
    fn maybe_get_mut(&mut self) -> Option<&mut T>;
    fn set_maybe(&mut self, maybe_value: Option<T>);
}

impl<H, T> MaybeHas<T> for H
where
    H: Has<T>,
{
    fn maybe_get(&self) -> Option<&T> {
        Some(self.get())
    }

    fn maybe_get_mut(&mut self) -> Option<&mut T> {
        Some(self.get_mut())
    }

    fn set_maybe(&mut self, maybe_value: Option<T>) {
        if let Some(value) = maybe_value {
            *self.get_mut() = value;
        }
    }
}
