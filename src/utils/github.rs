extern crate percent_encoding;
use crate::utils::get_service_name;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

// Used as part of the percent_encoding library
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_github_url(query: &str) -> String {
let github_dotcom = "https://github.com";
    if query == "gh" {
                github_dotcom.to_string()
    } else {
        // Assume the other match is "gh page"
        let encoded_query = utf8_percent_encode(&query[3..], FRAGMENT).to_string();
        let github_url = format!("{}/{}", github_dotcom, encoded_query);

        github_url
    }
}

pub fn construct_sp_github_url(query: &str) -> String {
    let sp_github_dotcom = "https://github.com/saltpay";
    if query == "ghs" {
        sp_github_dotcom.to_string()
    } else {
        // Assume the other match is "ghs tb"
        let encoded_query = utf8_percent_encode(&query[4..], FRAGMENT).to_string();
        format!("{}/{}", sp_github_dotcom, get_service_name(&encoded_query))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_github_profile_url_with_gh() {
        let fake_query = "gh";
        assert_eq!(construct_github_url(fake_query), "https://github.com");
    }

    #[test]
    fn test_construct_github_profile_url_with_repo_url() {
        let fake_query = "gh facebook";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/facebook"
        );
    }

    #[test]
    fn test_construct_github_search_url_with_repo_url() {
        let fake_query = "gh facebook/docusaurus";
        assert_eq!(
            construct_github_url(fake_query),
            "https://github.com/facebook/docusaurus"
        );
    }
}
