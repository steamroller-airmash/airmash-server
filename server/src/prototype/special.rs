use std::borrow::Cow;

/// Prototype for a boost effect similar to the predator boost.
///
/// # Example
/// The prototype for the predator boost looks like this:
/// ```
/// # use server::prototype::BoostPrototype;
/// # use std::borrow::Cow;
/// BoostPrototype {
///   name: Cow::Borrowed("boost"),
///   cost: 0.01,
///   speedup: 1.5,
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BoostPrototype {
  /// The name with which this special effect will be referred to.
  name: Cow<'static, str>,
  /// The rate at which boosting uses up energy in (energy/frame)
  cost: f32,
  /// A multiplier that multiplies both the maximum speed and the accelerating
  /// while the plane is boosting.
  speedup: f32,
}

impl BoostPrototype {
  pub const fn predator() -> Self {
    Self {
      name: Cow::Borrowed("boost"),
      cost: 0.01,
      speedup: 1.5,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub enum SpecialPrototype {
  Boost(BoostPrototype),
  Multishot { name: Cow<'static, str> },
  Repel { name: Cow<'static, str> },
  Strafe { name: Cow<'static, str> },
}

impl SpecialPrototype {
  /// The name with which to refer to this prototype.
  ///
  /// It can be accessed individually in each enum variant but this convenience
  /// method is here to avoid needing to write a big match statement every time
  /// that is done.
  pub fn name(&self) -> &str {
    let name = match self {
      Self::Boost(BoostPrototype { name, .. })
      | Self::Multishot { name, .. }
      | Self::Repel { name, .. }
      | Self::Strafe { name, .. } => name,
    };

    match name {
      Cow::Borrowed(name) => name,
      Cow::Owned(name) => name,
    }
  }
}
