extern crate percent_encoding;
use crate::utils::get_service_name;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library added comment
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_grafana_url(query: &str) -> String {
    if query == "gf" {
        "dummy/".to_string()
    } else {
        // Assume the other match is "gh page"
        let env = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let service = utf8_percent_encode(&query[7..], FRAGMENT).to_string();
        let grafana = format!("dummy={}", env, get_service_name(&service));
        // print!("{}",grafana);
        grafana
    }
}
