/*  Accessibility Scanner
 *
 * Conducts single page scans of urls.
 *
 * Sends POST Request to A11yWatch API
 * Captures results and sends to BigQuery
 * Written by: @theBoatyMcBoatFace
 * On: Jan 24, 2023
 */
 *
 *
 *
 *
 *
 *
 *
 *
 *
*/

use serde::{Deserialize, Serialize};
 use serde_json::{json, Value};
 use serde_yaml::{from_reader, from_str};
 use bigquery_rs::Bigquery;
 use rocket::{post, routes, Config, request::Form};
 use reqwest::Client;
 use std::fs::File;
 use std::io::Read;

 // Define structs to match the schema of the BigQuery table

 #[derive(Serialize, Deserialize, Debug)]
 struct Scan {
     id: String,
     created_at: String,
     updated_at: String,
     domain: String,
     url: String,
     success: bool,
     code: i32,
     online: bool,
     page_load_duration: i32,
     page_load_description: String,
     page_load_color: String,
     issues_total: i32,
     issues_error: i32,
     issues_warning: i32,
     issues_notice: i32,
     issues_score: i32,
 }

 #[derive(Serialize, Deserialize, Debug)]
 struct Issue {
     scan_id: String,
     url: String,
     domain: String,
     last_scan_date: String,
     code: String,
     issue_type: String,
     issue_type_code: i32,
      message: String,
      context: String,
      selector: String,
        runner: String,
        recurrence: i32,
        updated_at: String,
        created_at: String,
     }



#[post("/scan", data = "<scan>")]
fn scan(scan: Json<Scan>) -> &'static str {
    let client = Client::new();
    let bq = Bigquery::new("ca-a11y");

    // Make API request
    let resp = client
        .post("https://a11ywatch-backend.public-sector-a11y.app.civicactions.net/api/scan")
        .json(&scan)
        .header("Content-Type", "application/json")
        .send()
        .unwrap();

    // Parse response into json
    let scan_data: Value = serde_json::from_str(resp.text().unwrap().as_str()).unwrap();

    // Insert scan into BigQuery
    let scan_id = uuid::Uuid::new_v4().to_string();
    let new_scan = Scan {
        id: scan_id.to_string(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        domain: scan_data["data"]["domain"].as_str().unwrap().to_owned(),
        url: scan_data["data"]["url"].as_str().unwrap().to_owned(),
        success: scan_data["data"]["cdnConnected"].as_bool().unwrap(),
        code: scan_data["data"]["lastScanDate"].as_str().unwrap().to_owned(),
        online: scan_data["data"]["pageLoadTime"]["duration"].as_i64().unwrap() as i32,
        page_load_duration: scan_data["data"]["pageLoadTime"]["duration"].as_i64().unwrap() as i32,
        page_load_description: scan_data["data"]["pageLoadTime"]["durationFormated"]
            .as_str()
            .unwrap()
            .to_owned(),
        page_load_color: scan_data["data"]["pageLoadTime"]["color"].as_str().unwrap().to_owned(),
        issues_total: scan_data["data"]["issues"].as i32,
        issu

// Add the rest of the issues... sooo many issues....


issues_error: i32,
         issues_warning: i32,
         issues_notice: i32,
         issues_score: i32



    bq.query(&format!("INSERT INTO public_data.scans (id, created_at, updated_at, domain, url, success, code, online, page_load_duration, page_load_description, page_load_color, issues_total, issues_error, issues_warning, issues_notice, issues_score) VALUES ('{}', '{}', '{}', '{}', '{}', {}, {}, {}, {}, '{}', '{}', {}, {}, {}, {}, {})",
    scan_id.to_string(), Utc::now().to_rfc3339(), Utc::now().to_rfc3339(), scan_data["data"]["domain"].as_str().unwrap().to_owned(), scan_data["data"]["url"].as_str().unwrap().to_owned(), scan_data["data"]["success"].as_bool().unwrap(), scan_data["data"]["code"].as_i64().unwrap(), scan_data["data"]["online"].as_bool().unwrap(), scan_data["data"]["pageLoadTime"]["duration"].as_i64().unwrap() as i32, scan_data["data"]["pageLoadTime"]["durationFormated"].as_str().unwrap().to_owned(), scan_data["data"]["pageLoadTime"]["color"].as_str().unwrap().to_owned(), scan_data["data"]["issues"].as_array().unwrap().len(), scan_error_count, scan_warning_count, scan_notice_count, scan_score
         )).unwrap();
        for issue in scan_data["data"]["issues"].as_array().unwrap() {
        let issue_data = json!({
            "scan_id": scan_id,
            "url": issue["url"].as_str().unwrap(),
            "domain": issue["domain"].as_str().unwrap(),
            "last_scan_date": issue["lastScanDate"].as_str().unwrap(),
            "code": issue["code"].as_str().unwrap(),
            "issue_type": issue["type"].as_str().unwrap(),
            "issue_type_code": issue["typeCode"].as_i64().unwrap() as i32,
            "message": issue["message"].as_str().unwrap(),
            "context": issue["context"].as_str().unwrap(),
            "selector": issue["selector"].as_str().unwrap(),
            "runner": issue["runner"].as_str().unwrap(),
            "recurrence": issue["recurrence"].as_i64().unwrap() as i32,
            "created_at": Utc::now().to_rfc3339(),
            "updated_at": Utc::now().to_rfc3339(),
        });




// Finish breakdown of issue to BigQuery
//
//
//




