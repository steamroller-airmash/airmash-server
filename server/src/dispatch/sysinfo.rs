use specs::World;
use std::any::Any;

pub trait SystemDeps {
	fn dependencies() -> Vec<&'static str>;
}

pub trait SystemInfo {
	type Dependencies: SystemDeps;

	fn name() -> &'static str;
	fn new() -> Self;
	fn new_args(_args: Box<Any>) -> Self
	where
		Self: Sized,
	{
		Self::new()
	}

	fn static_setup(_: &mut World) {}
}

macro_rules! decl_tuple {
	{
		$(
			(
				$($param:ident),*
			);
		)*
	} => {
		$(
			impl<$($param,)*> SystemDeps for ($($param,)*)
			where $($param: SystemDeps,)*
			{
				fn dependencies() -> Vec<&'static str> {
					#[allow(unused_mut)]
					let mut deps = vec![];

					$(
						deps.append(&mut $param::dependencies());
					)*

					deps
				}
			}
		)*
	}
}

impl<T> SystemDeps for T
where
	T: SystemInfo,
{
	fn dependencies() -> Vec<&'static str> {
		vec![T::name()]
	}
}

// Alphabet pyramid, implement for every tuple up to 26 elements
decl_tuple! {
	();
	(A);
	(A, B);
	(A, B, C);
	(A, B, C, D);
	(A, B, C, D, E);
	(A, B, C, D, E, F);
	(A, B, C, D, E, F, G);
	(A, B, C, D, E, F, G, H);
	(A, B, C, D, E, F, G, H, I);
	(A, B, C, D, E, F, G, H, I, J);
	(A, B, C, D, E, F, G, H, I, J, K);
	(A, B, C, D, E, F, G, H, I, J, K, L);
	(A, B, C, D, E, F, G, H, I, J, K, L, M);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
	(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
}
