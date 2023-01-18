use reqwest;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::fs;
use std::io;
use std::path::PathBuf;
use csv::Reader;
use serde::Deserialize;

struct Covid {
    percent_positive_7_day: f32,
}

fn main(){
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build().expect("failed to build client");

    let counties = vec!["81"];
    for county in &counties{
        download_remote_file(&client, county);
        let file = fs::File::open("81.json").expect("file should open read only");
        let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");
        let first_name = json.get("integrated_county_timeseries_external_data").expect("file should have FirstName key");
        let inner_json = serde_json::from_str(first_name)?;
        let test = inner_json.get("percent_positive_7_day").expect("Key needed for positive");
        println!("{}", test)

    }
}

fn download_remote_file(c: &reqwest::blocking::Client, county: &str) {
    let fname = format!("{}.json", county);
    println!(" working on file data");
    let remote_file =  format!("https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_timeseries_fips_370{}_external",county);
    let content = c.get(remote_file).send().expect("failed to get uri").bytes().expect("failed to get bytes");
    let mut f = File::create(&fname).expect("failed to create fileß");
    f.write_all(&content).expect("failed to write to file");
}