use super::*;
use Builder;

pub fn register<'a, 'b>(builder: Builder<'a, 'b>) -> Builder<'a, 'b> {
	builder
		.with::<predator::SetBoostingFlag>()
		.with::<predator::SendEventBoost>()
		.with::<prowler::SetStealth>()
		.with::<prowler::SendEventStealth>()
		.with::<prowler::DestealthOnFire>()
		.with::<prowler::DestealthOnHit>()
		.with::<goliath::GoliathRepel>()
		.with::<goliath::SendEventRepel>()
		.with::<goliath::DestealthProwler>()
		.with::<tornado::Fire>()
}
