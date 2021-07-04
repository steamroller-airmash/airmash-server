macro_rules! def_wrappers {
  {
    $(
      $( #[$attr:meta] )*
      $( ##[nocopy $( $ncvis:ident )? ] )?
      $vis:vis type $nty:ident = $oty:ty ;
    )*
  } => {
    $(
      $( #[$attr] )*
      #[derive(Clone, Debug)]
      #[cfg_attr(all($( __server_disabled $( $ncvis )? )?), derive(Copy))]
      $vis struct $nty(pub $oty);

      impl From<$oty> for $nty {
        fn from(v: $oty) -> Self {
          Self(v)
        }
      }

      impl From<$nty> for $oty {
        fn from(v: $nty) -> Self {
          v.0
        }
      }

      impl ::std::ops::Deref for $nty {
        type Target = $oty;

        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }

      impl ::std::ops::DerefMut for $nty {
        fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
        }
      }
    )*
  };
}

macro_rules! def_wrapper_resources {
  {
    $(
      $( #[$attr:meta] )*
      $( ##[nocopy $( $ncvis:ident )? ] )?
      $vis:vis type $nty:ident = $oty:ty ;
    )*
  } => {
    $(
      $( #[$attr] )*
      #[derive(Clone, Debug, Default)]
      #[cfg_attr(all($( __server_disabled $( $ncvis )? )?), derive(Copy))]
      $vis struct $nty(pub $oty);

      impl From<$oty> for $nty {
        fn from(v: $oty) -> Self {
          Self(v)
        }
      }

      impl From<$nty> for $oty {
        fn from(v: $nty) -> Self {
          v.0
        }
      }

      impl ::std::ops::Deref for $nty {
        type Target = $oty;

        fn deref(&self) -> &Self::Target {
          &self.0
        }
      }

      impl ::std::ops::DerefMut for $nty {
        fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
        }
      }

      const _: () = {
        #[crate::handler]
        fn register_resource(
          _: &airmash_server::event::ServerStartup,
          world: &mut airmash_server::AirmashGame
        ) {
          if world.resources.get::<$nty>().is_none() {
            world.resources.insert(<$nty>::default());
          }
        }
      };
    )*
  };
}
