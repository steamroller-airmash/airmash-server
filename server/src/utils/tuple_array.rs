#[macro_export]
macro_rules! tuple_array {
	[ ] => { () };
	[$head:ty, $( $tail:ty ),*] => {
		($head, tuple_array![ $( tail:ty ),* ])
	};
}
