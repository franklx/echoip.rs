#!/bin/bash
GEOIP_LICENSE_KEY="IA801FFLLnaX3TQs"

mkdir -p geoip
for DB in GeoLite2-City GeoLite2-Country GeoLite2-ASN; do
    curl -fsSL -m 30 "https://download.maxmind.com/app/geoip_download?edition_id=${DB}&license_key=${GEOIP_LICENSE_KEY}&suffix=tar.gz" \
        | tar --wildcards --strip-components=1 -C ./geoip -xzf - '*.mmdb'
done
