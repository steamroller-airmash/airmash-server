
use protocol::include::FlagCode;

const COUNTRY_CODES: [Option<&'static str>; 126] = [
    None, 
    Some("SY"), 
    Some("TH"), 
    Some("TM"), 
    Some("TN"), 
    Some("TR"), 
    Some("TT"), 
    Some("TW"), 
    Some("TZ"), 
    Some("UA"), 
    Some("UN"), 
    Some("US"), 
    Some("UY"), 
    Some("UZ"), 
    Some("VE"), 
    Some("VN"), 
    Some("PR"), 
    Some("PT"), 
    Some("PY"), 
    Some("QA"), 
    Some("rainbow"), 
    Some("RO"), 
    Some("RS"), 
    Some("RU"), 
    Some("SA"), 
    Some("SE"), 
    Some("SG"), 
    Some("SI"), 
    Some("SK"), 
    Some("SM"), 
    Some("MK"), 
    Some("MO"), 
    Some("MT"), 
    Some("MX"), 
    Some("MY"), 
    Some("NG"), 
    Some("NL"), 
    Some("NO"), 
    Some("NP"), 
    Some("NZ"), 
    Some("OM"), 
    Some("PA"), 
    Some("PE"), 
    Some("JP"), 
    Some("KP"), 
    Some("KR"), 
    Some("KW"), 
    Some("KZ"), 
    Some("LB"), 
    Some("LI"), 
    Some("LK"), 
    Some("LT"), 
    Some("LU"), 
    Some("LV"), 
    Some("HN"), 
    Some("HR"), 
    Some("HU"), 
    Some("ID"), 
    Some("IE"), 
    Some("IL"), 
    Some("IM"), 
    Some("imperial"), 
    Some("IN"), 
    Some("IQ"), 
    Some("DE"), 
    Some("DK"), 
    Some("DO"), 
    Some("DZ"), 
    Some("EC"), 
    Some("EE"), 
    Some("EG"), 
    Some("ES"), 
    Some("EU"), 
    Some("BH"), 
    Some("BO"), 
    Some("BR"), 
    Some("BT"), 
    Some("BY"), 
    Some("CA"), 
    Some("CH"), 
    Some("AD"), 
    Some("AE"), 
    Some("AL"), 
    Some("AM"), 
    Some("CL"), 
    Some("AQ"), 
    Some("CN"), 
    Some("AR"), 
    Some("FI"), 
    Some("CO"), 
    Some("AT"), 
    Some("IR"), 
    Some("FR"), 
    Some("communist"), 
    Some("AU"), 
    Some("LY"), 
    Some("IS"), 
    Some("GB"), 
    Some("confederate"), 
    Some("AZ"), 
    Some("MA"), 
    Some("IT"), 
    Some("GE"), 
    Some("CR"), 
    Some("BA"), 
    Some("PH"), 
    Some("MC"), 
    Some("JM"), 
    Some("GR"), 
    Some("CU"), 
    Some("BD"), 
    Some("SO"), 
    Some("PK"), 
    Some("MD"), 
    Some("JO"), 
    Some("GT"), 
    Some("CY"), 
    Some("BE"), 
    Some("ZA"), 
    Some("SV"), 
    Some("PL"), 
    Some("ME"), 
    Some("jolly"), 
    Some("HK"), 
    Some("CZ"), 
    Some("BG")
];

// Pull in the PHF table
include!(concat!(env!("OUT_DIR"), "/flags-phf.rs"));

impl FlagCode {
    /// Returns the numeric value that corresponds 
    /// to the flag code. 
    /// 
    /// This is the value that is actually sent
    /// in the serialized packet and is used 
    /// in the client.
    /// 
    /// Credit to Bombita for determining the 
    /// flag -> number mapping. Sources 
    /// [here](https://gist.github.com/Molesmalo/5e6d51aef8558e193720c3d963aeb5b7).
    pub fn to_u16(self) -> u16 {
       u16::from(self)
    }

    /// Return the flag code that corresponds to 
    /// the numeric value, or `None` if there 
    /// is no corresponding flag.
    /// 
    /// Credit to Bombita for determining the 
    /// flag -> number mapping. Sources 
    /// [here](https://gist.github.com/Molesmalo/5e6d51aef8558e193720c3d963aeb5b7).
    pub fn from_u16(v: u16) -> Option<FlagCode> {
        FlagCode::try_from(v)
    }

    /// Get the ISO-2 code associated with
    /// the flag (or special codes for non-country 
    /// flags)
    pub fn to_str(self) -> &'static str {
        COUNTRY_CODES[self.to_u16() as usize].unwrap()
    }

    /// Get the flag code associated with an
    /// ISO-2 country code. (or special codes
    /// for non-country flags).
    pub fn from_str(v: &str) -> Option<Self> {
        FLAG_MAP.get::<str>(&v.to_uppercase()).map(|x| *x)
    }
}

impl Default for FlagCode {
    fn default() -> Self {
        return FlagCode::UnitedNations;
    }
}
