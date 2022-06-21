use std::fmt::Debug;

use cfg_if::cfg_if;

#[cfg(anydebug)]
mod with_spec {
  use std::fmt;

  pub(super) struct DebugAny<T>(pub T);

  impl<T> fmt::Debug for DebugAny<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      f.write_str("..")
    }
  }

  impl<T> fmt::Debug for DebugAny<T>
  where
    T: fmt::Debug,
  {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.0.fmt(f)
    }
  }
}

#[allow(unused_variables)]
pub(crate) fn debug_any<T>(value: &T) -> impl Debug + '_ {
  cfg_if! {
    if #[cfg(anydebug)] {
      self::with_spec::DebugAny(value)
    } else {
      std::any::type_name::<T>()
    }
  }
}
