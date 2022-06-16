use super::*;

macro_rules! declare_userdata {
  {
    $( struct $st:ident : $type:ty => [$( $name:ident ),* $(,)? ] ; )*
  } => {
    $(
      struct $st;
      impl ::rlua::UserData for $st {
        fn add_methods<'lua, T: ::rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
          $(
            methods.add_function(stringify!($name), |lua, ()| {
              serde_rlua::to_value(lua, &<$type>::$name())
            });
          )*
        }
      }
    )*
  }
}

fn patch_defaults<'lua>(lua: rlua::Context<'lua>) -> rlua::Result<rlua::Table<'lua>> {
  declare_userdata! {
    struct PlaneDefaults : PlanePrototype => [
      predator,
      tornado,
      mohawk,
      goliath,
      prowler,
    ];

    struct MissileDefaults : MissilePrototype => [
      predator,
      tornado,
      tornado_triple,
      mohawk,
      goliath,
      prowler,
    ];

    struct SpecialDefaults : SpecialPrototype => [
      none,
      boost,
      multishot,
      repel,
      strafe,
      stealth
    ];
  }

  let table = lua.create_table()?;
  table.set("plane", PlaneDefaults)?;
  table.set("missile", MissileDefaults)?;
  table.set("special", SpecialDefaults)?;

  Ok(table)
}

impl GamePrototype {
  /// Similar to [`patch`] but it instead returns the rlua [`Value`] object
  /// directly. This isn't usually needed but it is useful if you want more
  /// control over the deserialization process. For example, the export binary
  /// uses this method so that it can get some additional information on which
  /// part failed to deserialize and use that for better error messages.
  ///
  /// [`patch`]: crate::GamePrototype::patch
  /// [`Value`]: rlua::Value
  pub fn patch_direct<'lua>(
    &self,
    lua: rlua::Context<'lua>,
    script: &str,
  ) -> rlua::Result<rlua::Value<'lua>> {
    let globals = lua.globals();
    globals.set("data", serde_rlua::to_value(lua, self)?)?;
    globals.set("defaults", patch_defaults(lua)?)?;

    lua.load(script).exec()?;

    globals.get("data")
  }

  /// Run a LUA script which can modify this GamePrototype instance.
  ///
  /// The script itself is just a LUA file which modifies a few globals that are
  /// injected into its environment. These are
  /// - `data` - The serialized form of this `GamePrototype` instance. The
  ///   script will modify this and at the end it will be assigned back to
  ///   `self`.
  /// - `defaults` - A collection of user functions that will return various
  ///   builtin prototypes so that they can be modified. There are no stability
  ///   guarantees around the addition of new fields to any of the prototypes so
  ///   using these functions to get one of the buitin prototypes and modifying
  ///   them is the only reliable way to build completely new class types.
  pub fn patch(&mut self, script: &str) -> rlua::Result<()> {
    use rlua::{Lua, StdLib};

    let libs = StdLib::ALL_NO_DEBUG;
    let lua = Lua::new_with(libs);
    *self = lua.context(|ctx| -> rlua::Result<_> {
      Ok(serde_rlua::from_value(self.patch_direct(ctx, script)?)?)
    })?;

    Ok(())
  }
}
