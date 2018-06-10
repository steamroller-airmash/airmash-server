
use specs::*;
use protocol::serde_am::*;
use protocol::error::{SerError, DeError};

type Text = String;
type TextBig = String;

type Array<T> = Vec<T>;
type ArraySmall<T> = Vec<T>;

include!(concat!(env!("OUT_DIR"), "/protocol-spec.rs"));

