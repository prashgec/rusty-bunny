#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    //println!("You typed in: {}", cmd);
    let command = utils::get_command_from_query_string(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "ghs" => utils::github::construct_sp_github_url(&cmd),
        "gf"| "rp" => utils::grafana::construct_grafana_url(&cmd),
        "okta" => "https://saltpayco.okta.com/app/UserHome".to_string(),
        "bob" => "https://app.hibob.com/home".to_string(),
        "oc"|"oncall"|"call" => "https://teyaglobal.app.opsgenie.com/schedule/whoIsOnCall".to_string(),
        _ => utils::google::construct_google_search_url(&cmd)
    };
    Redirect::to(redirect_url)
}

fn main() {
    //rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite().mount("/", routes![index, search]).launch();
}
