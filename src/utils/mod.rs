pub mod google;
pub mod github;
pub mod twitter;
pub mod grafana;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        // We need to this to know where to slice the string
        let index_of_space = query_string.find(' ').unwrap_or(0);
        return &query_string[..index_of_space];
    }
    // Otherwise, return the query string as is
    &query_string
}

pub fn get_service_name(code: &str) -> &str {
    match code {
        "tb" => "transaction-block-manager",
        "papi" => "acquiring-payments-api",
        "tba" => "transaction-block-aux",
        "fx" => "acceptance-fx-api",
        "bin" => "acceptance-bin-service",
"tbj" => "transaction-block-janitor",
        _ => "",
    }
}