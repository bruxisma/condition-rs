//! This library provides [`condition::Condition`](#) a trait for
//! easier expression (and consumption) of features, toggles, checkboxes,
//! settings, options, or any other [*bivalent*][1] pair.
//!
//! NOTE: This crate will eventually support no-std, but does not at this time
//!
//! Conditions typically come in pairs. (e.g., `{Allow, Deny}`, `{Yes, No}`,
//! `{With, Without}`. This library provides several types by default. To
//! import them, use the [`prelude`](prelude/index.html) module. If you only want to use
//! the trait, simply import it.
//!
//! Use `impl Condition` anywhere you might take a boolean value. Then, use
//! any type that implements this condition.
//!
//! Within the function, branch off of the condition provided
//!
//! ```
//! use condition::prelude::*;
//!
//! #[derive(Condition)]
//! enum Answer {
//!   No,
//!   Yes
//! }
//!
//! pub fn verbose(v: impl Condition) { assert!(v.is(false)); }
//!
//! /* ... */
//!
//! use Answer::No;
//! verbose(No);
//!
//! ```
//!
//!
//! [1]: https://en.wikipedia.org/wiki/Principle_of_bivalence
//!

pub trait Condition: Sized {
  /// Checks if the Condition is in the same equivalent state as the given
  /// boolean. Everything else regarding a condition can be implemented in
  /// these terms
  fn is(&self, value: bool) -> bool;

  /// Alias function for `self.is(false)`
  #[inline]
  fn is_false (&self) -> bool { self.is(false) }
  /// Alias function for `self.is(true)`
  #[inline]
  fn is_true (&self) -> bool { self.is(true) }

  /// Returns `Some(())` if `self.is_true()`, otherwise returns `None`
  #[must_use]
  fn option (&self) -> Option<()> { if self.is(true) { Some(()) } else { None } }
  /// Returns `Ok(())` if `self.is_true()`, otherwise returns `Err(())`
  #[must_use]
  fn result (&self) -> Result<(), ()> { self.option().ok_or(()) }
}

impl<T> Condition for Option<T> {
  #[inline]
  fn is (&self, value: bool) -> bool { self.is_some().is(value) }
}

impl<T, E> Condition for Result<T, E> {
  #[inline]
  fn is (&self, value: bool) -> bool { self.is_ok().is(value) }
}

impl Condition for bool {
  #[inline]
  fn is (&self, value: bool) -> bool { *self == value }
}

#[cfg(feature = "std")]
impl Condition for std::process::ExitStatus {
  #[inline]
  fn is (&self, value: bool) -> bool { self.success().is(value) }
}

pub mod prelude {
  pub use ::condition_derive::*;
  pub use crate::Condition;
}

#[doc(hidden)]
#[cfg(test)]
mod test {
  use super::prelude::*;
  #[derive(Condition)]
  enum Answer {
    No,
    Yes,
  }

  #[test]
  fn test_derive () {
    let no: Answer = Answer::No;
    let yes: Answer = Answer::Yes;
    assert!(no.is(false));
    assert!(yes.is(true));
  }

  #[test]
  fn test_option_condition () {
    assert!(Option::<()>::None.is(false));
    assert!(Option::Some(()).is(true));
  }

  #[test]
  fn test_result_condition () {
    assert!(Result::<(),()>::Err(()).is(false));
    assert!(Result::<(),()>::Ok(()).is(true));
  }

  #[test]
  fn test_bool_condition () {
    assert!(false.is(false));
    assert!(true.is(true));
  }
  #[test]
  fn test_into_option () {
    assert_eq!(Option::None, false.option());
    assert_eq!(Some(()), true.option());
  }

  #[test]
  fn test_into_result () {
    assert_eq!(Result::Err(()), false.result());
    assert_eq!(Result::Ok(()), true.result());
  }
}
