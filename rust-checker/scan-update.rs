/*  Accessibility Scanner
 *
 * Conducts single page scans of urls.
 *
 * Sends POST Request to A11yWatch API
 * Captures results and sends to BigQuery
 * Written by: @theBoatyMcBoatFace
 * On: Jan 24, 2023
 *
 *      So... upserting is a little complex
 *      Moving the v1 of /scan over here to work on
 *      This script will only insert new issues and update existing issues with the updates_at
 *      every issue will be checked against the url and context
 *      if there is a match, it will update, if not, it will create
 *      Too complex for v1, moving to v2 :)
 */

use serde::{Deserialize, Serialize};
 use serde_json::{json, Value};
 use serde_yaml::{from_reader, from_str};
 use bigquery_rs::Bigquery;
 use rocket::{post, routes, Config, request::Form};
 use reqwest::Client;
 use std::fs::File;
 use std::io::Read;

 // Define structs to match the schema of the BigQuery tables

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

#[post("/scan", data = "<scan_form>")]
fn scan(scan_form: Form<ScanForm>) -> String {
    // Read the maps.yaml file
    let mut file = File::open("maps.yaml").expect("Failed to open maps.yaml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read maps.yaml");
    let maps: Value = from_str(&contents).expect("Failed to parse maps.yaml");

    // Send the POST request to the external API
         let client = Client::new();
         let resp = client
             .post("https://a11ywatch-backend.public-sector-a11y.app.civicactions.net/api/scan")
             .json(&scan_form.0)
             .send()
             .expect("Failed to send POST request");
         let json: Value = resp.json().expect("Failed to parse JSON response");

    // Extract the relevant data from the response
         let scan_data = json["data"].clone();
         let issues = json["data"]["issues"].clone();

// Prepare data to insert into the 'scans' table
    let scan = Scan {
        id: "".to_owned(),
        created_at: "".to_owned(),
        updated_at: "".to_owned(),
        domain: scan_data["domain"].as_str().unwrap().to_owned(),
        url: scan_data["url"].as_str().unwrap().to_owned(),
        success: scan_data["success"].as_bool().unwrap(),
        code: scan_data["code"].as_i64().unwrap() as i32,
        online: scan_data["online"].as_bool().unwrap(),
        page_load_duration: scan_data["pageLoadTime"]["duration"].as_i64().unwrap() as i32,
        page_load_description: scan_data["pageLoadTime"]["durationFormated"]
            .as_str()
            .unwrap()
            .to_owned(),
        page_load_color: scan_data["pageLoadTime"]["color"].as
//
//
//
// to-do: fix this area. Params off
// Need to step away and come back to this area.
//
//

#[post("/scan", data = "<scan>")]
fn scan(scan: Json<Scan>) -> &'static str {
    let client = Client::new();
    let bq = Bigquery::new("ca-a11y");

    let scan_response = client.post("https://a11ywatch-backend.public-sector-a11y.app.civicactions.net/api/scan")
        .json(&scan)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .unwrap();
//
//
// Formatting is a hot mess. Fix when you get back from walk
//
//
    let scan_data: serde_json::Value = serde_json::from_str(scan_response.text().unwrap().as_str()).unwrap();

    let scan_id = Uuid::new_v4();

    let new_scan = Scan {
        id: scan_id.to_string(),
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
        domain: scan_data["data"]["domain"].as_str().unwrap().to_owned(),
        url: scan_data["data"]["url"].

        success: scan_data["data"]["cdnConnected"].as_bool().unwrap(),
            code: scan_data["data"]["httpCode"].as_i64().unwrap() as i32,
            online: scan_data["data"]["online"].as_bool().unwrap(),
            page_load_duration: scan_data["data"]["pageLoadTime"]["duration"].as_i64().unwrap() as i32,
            page_load_description: scan_data["data"]["pageLoadTime"]["durationFormated"].as_str().unwrap().to_owned(),
            page_load_color: scan_data["data"]["pageLoadTime"]["color"].as_str().unwrap().to_owned(),
            issues_total: scan_data["data"]["issues"].len() as i32,
            issues_error: scan_data["data"]["issues"].iter().filter(|x| x["type"] == "error").count() as i32,
            issues_warning: scan_data["data"]["issues"].iter().filter(|x| x["type"] == "warning").count() as i32,
            issues_notice: scan_data["data"]["issues"].iter().filter(|x| x["type"] == "notice").count() as i32,
            issues_score: scan_data["data"]["issues"].iter().map(|x| x["typeCode"].as_i64().unwrap() as i32).sum::<i32>()
        };
//
// Yeet data into BigQuery
//
        bq.query(&format!("INSERT INTO public_data.scans (id, created_at, updated_at, domain, url, success, code, online, page_load_duration, page_load_description, page_load_color, issues_total, issues_error, issues_warning, issues_notice, issues_score")
        };
    //
    //
    //      ~~You messed up the list of fields. Fix me~~
    //
    //          What am I missing below?


        let new_scan = Scan {
            id: scan_id.to_string(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            domain: scan_data["data"]["domain"].as_str().unwrap().to_owned(),
            url: scan_data["data"]["url"].as_str().unwrap().to_owned(),
            success: scan_data["data"]["cdnConnected"].as_bool().unwrap(),
            code: scan_data["data"]["code"].as_i64().unwrap() as i32,
            online: scan_data["data"]["online"].as_bool().unwrap(),
            page_load_duration: scan_data["data"]["pageLoadTime"]["duration"].as_i64().unwrap() as i32,
            page_load_description: scan_data["data"]["pageLoadTime"]["durationFormated"].as_str().unwrap().to_owned(),
            page_load_color: scan_data["data"]["pageLoadTime"]["color"].as_str().unwrap().to_owned(),
            issues_total: scan_data["data"]["issues"].as_array().unwrap().len() as i32,
        };
//
//
//
//
//
        let insert_scan = bq.query(&format!("INSERT INTO public_data.scans (id, created_at, updated_at, domain, url, success, code, online, page_load_duration, page_load_description, page_load_color, issues_total) VALUES ('{}', '{}', '{}', '{}', '{}', {}, {}, {}, {}, '{}', '{}', {})", new_scan.id, new_scan.created_at, new_scan.updated_at, new_scan.domain, new_scan.url, new_scan.success, new_scan.code, new_scan.online, new_scan.page_load_duration, new_scan.page_load_description, new_scan.page_load_color, new_scan.issues_total)).unwrap();

        // Loop through each issue in the response and insert into BigQuery

// To Do


//
//      ~~Pull in SQL from Checky The Checker~~
//
//          We don't want duplicate issues if they ahve already been reported
//          As issues are
//

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



let existing_issue = bq
    .select(&format!(
        "SELECT * FROM public_data.scan_issues WHERE url='{}' and context='{}'",
        issue_data["url"], issue_data["context"]
    ))
    .unwrap();



if existing_issue.is_empty() {
    bq.query(&format!(
        "INSERT INTO public_data.scan_issues (scan_id, url, domain, last_scan_date, code, issue_type, issue_type_code, message, context, selector, runner, recurrence, created_at, updated_at) VALUES ('{}','{}','{}','{}','{}','{}',{},'{}','{}','{}','{}',{},'{}','{}')",
        issue_data["scan_id"],
        issue_data["url"],
        issue_data["domain"],
        issue_data["last_scan_date"],
        issue_data["code"],
        issue_data["issue_type"],
        issue_data["issue_type_code"],
        issue_data["message"],
        issue_data["context"],
        issue_data["selector"],
        issue_data["runner"],
        issue_data["recurrence"],
        issue_data["created_at"],
        issue_data["updated_at"]
    )).unwrap();
} else {
    bq.query