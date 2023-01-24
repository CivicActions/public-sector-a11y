/*  Domain Up Checker
 * Sends GET Request to Website
 * If non 200 response, marks site as site_error = True
 * Written by: @theBoatyMcBoatFace
 * On: Jan 24, 2023
 *
 *      A simple server-side application that checks if a website is up
 *
 *      Domains set as scan_active in the ca_ally.domains table are checked for reachability
 *      If the site is reachable, the next domain is checked
 *      If the site is not reachable, the TRUE value is set for the site_error column
 *
 *
 *
 *
 *
 *
 */

extern crate rocket;
extern crate rocket_contrib;
extern crate reqwest;
extern crate bigquery_rs;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate log;

use rocket::{routes, Config};
use rocket_contrib::json::Json;
use reqwest::Client;
use bigquery_rs::Bigquery;
use serde_derive::{Serialize, Deserialize};
use log::{info, error};
use std::env;

// Authenticate with Google Cloud
fn main() {
let google_cloud_key = env::var("${{ secrets.GOOGLE_CLOUD_KEY }}").expect("GOOGLE_CLOUD_KEY must be set");
let bq = Bigquery::new_with_credentials("ca-a11y", &google_cloud_key);


#[derive(Serialize, Deserialize)]
 struct Domain {
     id: String,
     agency_id: String,
     name: String,
     site_error: bool,
     scan_active: bool,
     domain: String,
 }

 #[derive(Serialize, Deserialize)]
 struct DomainCheckResult {
     domain: String,
     site_error: bool,
 }

// Handle a POST request to the "/up" route


#[post("/up")]
fn up() -> Json<Vec<DomainCheckResult>> {
    // Create a new HTTP client
    let client = Client::new();
    // Connect to Google BigQuery
    let bq = Bigquery::new("ca-a11y");
    // Select all domains where "scan_active" is TRUE
    let domains = match bq.select("SELECT * FROM public_data.domains where scan_active = true") {
        Ok(domains) => domains,
        Err(e) => {
            error!("Error getting domains from BigQuery: {:?}", e);
            vec![]
        }
    };
    let mut results = vec![];
    // Check if the website is publicly reachable
    for domain in domains {
        // Send GET request to domain
        let resp = match client.get(&domain.domain).send() {
            Ok(resp) => resp,
            Err(e) => {
                error!("Error getting response from domain: {:?}", e);
                continue;
            }
        };
        let site_error = !resp.status().is_success();
        if site_error {
            match bq.query(&format!("UPDATE public_data.domains SET site_error = true WHERE id = '{}'", domain.id)) {
                Ok(_) => {},
                Err(e) => error!("Error updating domain in BigQuery: {:?}", e),
            }
        }
        results.push(DomainCheckResult{domain: domain.domain, site_error});
    }
    // return a JSON object containing a list of "DomainCheckResult" structs
    Json(results)
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



