
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
