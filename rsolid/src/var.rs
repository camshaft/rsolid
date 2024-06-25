use core::{fmt, marker::PhantomData};

// TODO move these to a toml file
pub const fn preview() -> Var<bool> {
    Var::new("$preview")
}

pub const fn t() -> Var<f64> {
    Var::new("$t")
}

pub struct Var<T>(&'static str, PhantomData<T>);

impl<T> Var<T> {
    pub const fn new(name: &'static str) -> Var<T> {
        Var(name, PhantomData)
    }
}

impl<T> fmt::Debug for Var<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} as {}", self.0, core::any::type_name::<T>())
    }
}

impl<T> Clone for Var<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Var<T> {}
