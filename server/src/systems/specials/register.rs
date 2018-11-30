use super::*;
use Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with::<predator::SetBoostingFlag>()
		.with::<predator::SendEventBoost>()
		.with::<prowler::SetStealth>()
		.with_handler::<prowler::SendEventStealth>()
		.with_handler::<prowler::DestealthOnFire>()
		.with_handler::<prowler::DestealthOnHit>()
		.with::<goliath::GoliathRepel>()
		.with_handler::<goliath::SendEventRepel>()
		.with_handler::<goliath::DestealthProwler>()
		.with::<tornado::Fire>()
}
