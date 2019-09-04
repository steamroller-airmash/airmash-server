use shred::{Accessor, DynamicSystemData, ResourceId, System, World};

use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use crate::dispatch::sysinfo::*;
use crate::utils::DebugAdapter;

pub struct SystemWrapper<T>(pub T);

pub struct SystemWrapperData<'a, T>
where
	T: System<'a>,
	T::SystemData: DynamicSystemData<'a>,
{
	pub inner: T::SystemData,
	pub debug: DebugAdapter<'a>,
}

pub struct CombinedAccessor<A, B>(A, B);

impl<A, B> Accessor for CombinedAccessor<A, B>
where
	A: Accessor,
	B: Accessor,
{
	fn try_new() -> Option<Self> {
		Some(CombinedAccessor(A::try_new()?, B::try_new()?))
	}

	fn reads(&self) -> Vec<ResourceId> {
		let mut res = self.0.reads();
		res.append(&mut self.1.reads());
		res
	}

	fn writes(&self) -> Vec<ResourceId> {
		let mut res = self.0.writes();
		res.append(&mut self.1.writes());
		res
	}
}

impl<'a, T> DynamicSystemData<'a> for SystemWrapperData<'a, T>
where
	T: System<'a>,
	T::SystemData: DynamicSystemData<'a>,
{
	type Accessor = CombinedAccessor<
		<<T as System<'a>>::SystemData as DynamicSystemData<'a>>::Accessor,
		<DebugAdapter<'a> as DynamicSystemData<'a>>::Accessor,
	>;

	fn setup(acc: &Self::Accessor, res: &mut World) {
		T::SystemData::setup(&acc.0, res);
		DebugAdapter::setup(&acc.1, res);
	}

	fn fetch(acc: &Self::Accessor, res: &'a World) -> Self {
		Self {
			inner: T::SystemData::fetch(&acc.0, res),
			debug: DebugAdapter::fetch(&acc.1, res),
		}
	}
}

impl<'a, T> System<'a> for SystemWrapper<T>
where
	T: System<'a> + SystemInfo + Send,
	<T as System<'a>>::SystemData: DynamicSystemData<'a>,
{
	type SystemData = SystemWrapperData<'a, T>;

	fn setup(&mut self, res: &mut World) {
		self.0.setup(res);
	}

	fn run(&mut self, data: Self::SystemData) {
		let SystemWrapperData { inner, mut debug } = data;

		super::DEBUG_ADAPTER.with(|f| {
			// UNSAFE: This casts away the lifetime
			*f.borrow_mut() = &mut debug as *mut _ as *mut () as *mut _
		});

		let res = catch_unwind(AssertUnwindSafe(|| {
			self.0.run(inner);
		}));

		super::DEBUG_ADAPTER.with(|f| {
			*f.borrow_mut() = std::ptr::null_mut();
		});

		if let Err(e) = res {
			resume_unwind(e);
		}
	}
}
