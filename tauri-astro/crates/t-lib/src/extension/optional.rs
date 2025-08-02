pub trait Optional<T> {
    fn value(self) -> T;
}

impl<T> Optional<T> for Option<T>
where
    T: Default,
{
    fn value(self) -> T {
        match self {
            Some(value) => value,
            None => T::default(),
        }
    }
}
