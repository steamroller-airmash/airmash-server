
#[cfg(not(feature="geoip"))]
mod default_geoip {
	use std::net::IpAddr;
	use protocol::FlagCode;

	/// An empty lookup that always returns None
	fn locate(_: &IpAddr) -> Option<FlagCode> {
		None
	}
}

#[cfg(feature="geoip")]
mod full_geoip {
	extern crate geolocate_ip;

	use std::net::IpAddr;
	use protocol::FlagCode;

	/// Look up ISO-2 country code
	fn locate(addr: &IpAddr) -> Option<FlagCode> {
		match *addr {
			IpAddr::V4(a) => match geolocate_ip::lookup_ip(&a) {
				Some(s) => FlagCode::from_str(s),
				None => None,
			},
			// IP lookups not done for Ipv6 addresses yet
			IpAddr::V6(_) => None
		}
	}
}

#[cfg(feature="geoip")]
pub use self::full_geoip::*;

#[cfg(not(feature="geoip"))]
pub use self::default_geoip::*;
