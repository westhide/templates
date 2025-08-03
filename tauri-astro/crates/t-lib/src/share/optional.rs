pub trait DefaultOption<T> {
    fn into(self) -> T;
}

impl<T> DefaultOption<T> for Option<T>
where
    T: Default,
{
    fn into(self) -> T {
        match self {
            Some(value) => value,
            None => T::default(),
        }
    }
}
