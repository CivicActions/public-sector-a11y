/*  Domain Up Checker
 * Sends GET Request to Website
 * If non 200 response, marks site as site_error = True
 * Written by: @theBoatyMcBoatFace
 * On: Jan 24, 2023
 */

use rocket::{routes, Config};
use rocket_contrib::json::Json;
use reqwest::Client;
use bigquery_rs::Bigquery;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Domain {
    id: String,
    agency_id: String,
    name: String,
    site_error: bool,
    scan_active: bool,
    domain: String,
}

#[post("/up", data = "<domain>")]
fn up(domain: Json<Domain>) -> &'static str {
    // Create a new http client
    let client = Client::new();
    // Connect to Google BigQuery
    let bq = Bigquery::new("ca-a11y");
    // Select all domains where "scan_active" is TRUE.
    let domains = bq.select("SELECT * FROM public_data.domains where scan_active = true")
        .unwrap();
    let mut site_error = false;
    // Check if the website is publicaly reachable.
    for domain in domains {
        // Send GET request to domain
        let resp = client.get(&domain.domain).send();
        if resp.is_err() {
            // if the response is not successful
            site_error = true;
            // Modify the BigQuery table (domains) to set the site_error bool to TRUE.
            bq.query(&format!("UPDATE public_data.domains SET site_error = true WHERE id = '{}'", domain.id))
                .unwrap();
        }
    }
    if site_error {
        "True"
    } else {
        "False"
    }
}

fn main() {
    let config = Config::build(rocket::config::Environment::Staging)
        .address("0.0.0.0")
        .port(8080)
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![up])
        .launch();
}