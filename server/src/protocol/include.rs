use protocol::error::{DeError, SerError};
use protocol::serde_am::*;
use specs::*;

type Array<T> = Vec<T>;
type ArraySmall<T> = Vec<T>;

include!(concat!(env!("OUT_DIR"), "/protocol-spec.rs"));
