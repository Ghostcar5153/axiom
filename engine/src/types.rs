/// Taint marker for values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
 pub enum Taint {
     Clean,
     Tainted,
 }

/// Wrap any T to carry a taint flag.
#[derive(Debug, Clone)]
 pub struct Value<T> {
     pub inner: T,
     pub taint: Taint,
 }

 impl<T> Value<T> {
     pub fn new(inner: T, taint: Taint) -> Self {
         Self { inner, taint }
     }
 }
