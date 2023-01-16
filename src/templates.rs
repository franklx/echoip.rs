use maud::{html, PreEscaped, DOCTYPE, Render, Markup};

use crate::model::Index;

fn tr_some<T: Render>(var: &Option<T>, label: &str) -> Markup {
    html! {
        @if let Some(var) = var {
            tr {
                th scope="row" {
                    (label)
                }
                td {
                    (var)
                }
            }
        }
    }
}

fn tr_any<T: Default + PartialEq + Render>(var: &T, label: &str) -> Markup {
    html! {
        @if T::default() != *var {
            tr {
                th scope="row" {
                    (label)
                }
                td {
                    (var)
                }
            }
        }
    }
}

fn tr_bool<T: Default + PartialEq>(var: &T, label: &str) -> Markup {
    html! {
        tr {
            th scope="row" {
                (label)
            }
            td {
                @if T::default() != *var {
                    "false"
                }
                else {
                    "true"
                }
            }
        }
    }
}

pub fn index_html(data: Index, json: String) -> String {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title {
                    (format!("What is my ip address? \u{2014} {}", data.host))
                }
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content=(format!("{} • What is my ip address? — The best tool to find your own ip address, and information about it.", data.host));
                link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
                link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png";
                link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png";
                link rel="manifest" href="/site.webmanifest";
                link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5";
                meta name="msapplication-TileColor" content="#da532c";
                meta name="theme-color" content="#ffffff";
                link rel="canonical" href="https://ifconfig.co/";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link href="https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,400;0,700;1,400&display=swap" rel="stylesheet";
                link rel="stylesheet" href="pure-min.css";
                link rel="stylesheet" href="grids-responsive-min.css";
                link rel="stylesheet" href="/css/style.css";
                script type="text/javascript" {
                    (PreEscaped(format!(r#"const HOST = "{}"; const JSON_OBJ = '{}';"#, data.host, json)))
                }
            }
            body {
                .content {
                    .pure-g.gutters.center {
                        ."pure-u-1" {
                            .l-box {
                                h1 {
                                    (format!("{} \u{2014} What is my ip address?", data.host))
                                }
                                p {
                                    code.ip {
                                        (data.ip)
                                    }
                                }
                                p {
                                    "The best tool to find your own ip address, and information about it."
                                }
                            }
                        }
                    }
                    .pure-g.gutters.center {
                        /*  COLUMN 1  */
                        ."pure-u-1"."pure-u-md-1-2".col {
                            .l-box {
                                h2 {
                                    "What do we know about this ip address?"
                                }
                                table.info-table {
                                    tbody {
                                        (tr_any(&data.ip, "ip\u{a0}address"))
                                        (tr_any(&data.decimal_ip, "ip\u{a0}address (decimal)"))
                                        @if let Some(ref geo_info) = data.geo_info {
                                            (tr_some(&geo_info.country_name, "Country"))
                                            (tr_some(&geo_info.country_iso, "Country (ISO code)"))
                                            (tr_bool(&geo_info.country_in_eu, "In EU?"))
                                            (tr_some(&geo_info.region_name, "Region"))
                                            (tr_some(&geo_info.region_code, "Region\u{a0}code"))
                                            (tr_some(&geo_info.metro_code, "Metro code"))
                                            (tr_some(&geo_info.postal_code, "Postal\u{a0}code"))
                                            (tr_some(&geo_info.city, "City"))
                                            (tr_some(&geo_info.latitude, "Latitude"))
                                            (tr_some(&geo_info.longitude, "Longitude"))
                                            (tr_some(&geo_info.timezone, "Timezone"))
                                            (tr_some(&geo_info.asn, "ASN"))
                                            (tr_some(&geo_info.asn_org, "ASN (organization)"))
                                        }
                                        @if let Some(ref user_info) = data.user_info {
                                            (tr_any(&user_info.hostname, "Hostname"))
                                            (tr_any(&user_info.user_agent, "User\u{a0}agent"))
                                            (tr_any(&user_info.user_agent_comment, "User\u{a0}agent: Comment"))
                                            (tr_any(&user_info.user_agent_raw, "User\u{a0}agent: Raw"))
                                        }
                                    }
                                }
                                p {
                                    "This information is provided from the GeoLite2 database created by MaxMind, available from "
                                    a href="https://www.maxmind.com" {
                                        "www.maxmind.com"
                                    }
                                }
                            }
                        }
                        /*  COLUMN 2  */
                        ."pure-u-1"."pure-u-md-1-2" {
                            .l-box {
                                h2 {
                                    "How do I get this programmatically?"
                                }
                                p {
                                    "With the widget below you can build your query, and see what the result will look like."
                                }
                                .pure-form {
                                    /*  COMMAND WIDGET  */
                                    .input-buttons {
                                        button.pure-button.widget-select name="ip" onclick="changeInput(this.name, this)" {
                                            "ip"
                                        }
                                        @if let Some(ref geo_info) = data.geo_info {
                                            @if geo_info.country_name.is_some() {
                                                button.pure-button.widget-select name="country" onclick="changeInput(this.name, this)" {
                                                    "country"
                                                }
                                                button.pure-button.widget-select name="country-iso" onclick="changeInput(this.name, this)" {
                                                    "country-iso"
                                                }
                                            }
                                            @if geo_info.city.is_some() {
                                                button.pure-button.widget-select name="city" onclick="changeInput(this.name, this)" {
                                                    "city"
                                                }
                                            }
                                            @if geo_info.asn.is_some() {
                                                button.pure-button.widget-select name="asn" onclick="changeInput(this.name, this)" {
                                                    "asn"
                                                }
                                            }
                                        }
                                        button.pure-button.widget-select name="json" onclick="changeInput(this.name, this)" {
                                            "json"
                                        }
                                        button.pure-button.widget-select name="port" onclick="changeInput(this.name, this)" {
                                            "port"
                                        }
                                        input #portInput.narrow-input.pure-input type="number" min="1" max="65535" value="8080" placeholder="8080" onchange="updatePort(this.value)" {}
                                    }
                                    .widgetbox.input {
                                        code #command {}
                                    }
                                    #output.widgetbox.output {}
                                    form.pure-form.input-buttons {
                                        fieldset {
                                            label for="ipInput" {
                                                "Check another ip (optional) "
                                                input #ipInput type="text" placeholder="ip to query" onkeyup="updateIP(this.value)";
                                            }
                                            button.pure-button type="button" onclick="navigate()" {
                                                "Open"
                                            }
                                        }
                                    }
                                }
                                @if let Some(ref geo_info) = data.geo_info {
                                    @if let (Some(ref bbox), Some(ref lat), Some(ref lng)) = (&geo_info.bbox, geo_info.latitude, geo_info.longitude) {
                                        ."pure-u-1"."pure-u-md-1-1" {
                                            h2 {
                                                "Map"
                                            }
                                            iframe width="100%" height="350" frameborder="0" scrolling="no" marginheight="0" marginwidth="0"
                                                src=(
                                                    PreEscaped(format!("https://www.openstreetmap.org/export/embed.html?bbox={bx0}%2C{bx1}%2C{bx2}%2C{bx3}&layer=mapnik&marker={mk0}%2C{mk1}",
                                                        bx0=bbox.l, bx1=bbox.b, bx2=bbox.r, bx3=bbox.t,
                                                        mk0=lat, mk1=lng)
                                                )) {}
                                        }
                                    }
                                }
                                /*  FAQ  */
                                .FAQ {
                                    h2 {
                                        "FAQ"
                                    }
                                    h3 {
                                        "How do I force IPv4 or IPv6 lookup?"
                                    }
                                    p {
                                        "IPv4 or IPv6 can be forced by passing the appropriate flag to your client, e.g "
                                        code {
                                            "curl -L4"
                                        }
                                        " or "
                                        code {
                                            "curl -L6"
                                        }
                                        "."
                                    }
                                    h3 {
                                        "Can I force getting JSON?"
                                    }
                                    p {
                                        "Setting the "
                                        code {
                                            "Accept: application/json"
                                        }
                                        " header works as expected."
                                    }
                                    h3 {
                                        "Is automated use of this service permitted?"
                                    }
                                    p {
                                        "Yes, as long as the rate limit is respected. The rate limit is in place to ensure a fair service for all."
                                    }
                                    p {
                                        em {
                                            "Please limit automated requests to 1 request per minute"
                                        }
                                        ". No guarantee is made for requests that exceed this limit. They may be rate-limited, with a 429 status code, or dropped entirely."
                                    }
                                    h3 {
                                        "Can I run my own service?"
                                    }
                                    p {
                                        "Yes, the source code and documentation is available on "
                                        a href="https://github.com/flucchini/echoip.rs" {
                                            "GitHub"
                                        }
                                        "."
                                    }
                                }
                            }
                        }
                    }
                }
                script type="text/javascript" src="/js/main.js" {}
            }
        }
    }.into_string()
}

pub fn err500_html() -> String {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta http-equiv="X-UA-Compatible" content="ie=edge";
                title {
                    "Ooops (500)"
                }
                link rel="stylesheet" href="pure-min.css";
                link rel="stylesheet" href="grids-responsive-min.css";
                link rel="stylesheet" href="/css/style.css";
            }
            body {
                .container {
                    .row {
                        h1 {
                            "Ooops ..."
                        }
                        p {
                            "How embarrassing!"
                        }
                        p {
                            "Looks like something weird happened while processing your request."
                        }
                        p {
                            "Please try again in a few moments."
                        }
                    }
                }
            }
        }
    }.into_string()
}
