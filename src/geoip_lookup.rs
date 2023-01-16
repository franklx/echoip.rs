use std::{net::IpAddr};

use maxminddb::{geoip2, Reader};

use crate::{error::EchoIpError, model::{GeoInfo, BBox}};

trait OwnedOption {
    fn owned(&self) -> Option<String>;
}

impl OwnedOption for Option<&str> {
    fn owned(&self) -> Option<String> {
        self.map(|s| s.to_owned())
    }
}

impl OwnedOption for Option<&&str> {
    fn owned(&self) -> Option<String> {
        self.map(|s| s.to_owned().to_owned())
    }
}


pub struct GeoipLookup {
    city_reader: Reader<Vec<u8>>,
    asn_reader:  Reader<Vec<u8>>,
}
impl GeoipLookup {
    pub fn new() -> Self {
        Self {
            // TODO these throw errors if the files are missing
            city_reader: Reader::open_readfile("./geoip/GeoLite2-City.mmdb").unwrap(),
            asn_reader:  Reader::open_readfile("./geoip/GeoLite2-ASN.mmdb").unwrap(),
        }
    }

    pub fn lookup_geo_for_ip(&self, ip_addr: IpAddr) -> Result<GeoInfo, EchoIpError> {
        let geoip_city: geoip2::City = self.city_reader.lookup::<geoip2::City>(ip_addr)
            .map_err(|err| EchoIpError::MaxMindDbFailed { source: err })?;
        let geoip_asn: geoip2::Asn = self.asn_reader.lookup::<geoip2::Asn>(ip_addr)
            .map_err(|err| EchoIpError::MaxMindDbFailed { source: err })?;

        // TODO if the IP cannot be found, insert some dummy data, or redirect to a different page?

        let region = (|| geoip_city.subdivisions.as_ref()?.get(0))();
        let country = geoip_city.country.as_ref();
        let location = geoip_city.location.as_ref();

        let latitude = (|| location?.latitude)();
        let longitude = (|| location?.longitude)();

        Ok(GeoInfo {
            country_name  : (|| country?.names.as_ref()?.get("en").owned())(),
            country_iso   : (|| country?.iso_code.owned())(),
            country_in_eu : (|| country?.is_in_european_union)(),
            region_name   : (|| region?.names.as_ref()?.get("en").owned())(),
            region_code   : (|| region?.iso_code.owned())(),
            city          : (|| geoip_city.city?.names?.get("en").owned())(),
            metro_code    : (|| location?.metro_code)(),
            latitude,
            longitude,
            timezone      : (|| location?.time_zone.owned())(),
            postal_code   : (|| geoip_city.postal?.code.iter().next().owned())(),

            asn           : geoip_asn.autonomous_system_number.map(|asn| format!("AS{asn}")),
            asn_org       : geoip_asn.autonomous_system_organization.owned(),

            bbox          : (|| Some(
                BBox {
                    t: latitude? + 0.05,
                    r: longitude? + 0.05,
                    b: latitude? - 0.05,
                    l: longitude? - 0.05,
                }
            ))(),
        })
    }
}
