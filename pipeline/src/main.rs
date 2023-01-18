use reqwest;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use std::fs;
use std::io;
use std::path::PathBuf;
use csv::Reader;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use serde_json::from_reader;
use serde;
use serde_json::json;
use crate::serde::Deserializer;

#[derive(Serialize, Deserialize, Debug)]
pub struct L1 {
    pub runid: u32,
    pub integrated_county_timeseries_external_data: Vec<L2>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct L2 {
    fips_code: i32,
    state: String,
    state_name: String,
    county: String,
    cases_7_day_count_change: Option<f64>,
    deaths_7_day_count_change: String,
    new_test_results_reported_7_day_rolling_average:  Option<f64>,
    percent_positive_7_day:  Option<f64>,
    admissions_covid_confirmed_last_7_days_per_100k_population: Option<f64>,
    percent_adult_inpatient_beds_used_confirmed_covid: Option<f64>,
    percent_adult_icu_beds_used_confirmed_covid: Option<f64>,
    date:String,
    report_date_window_start:String,
    report_date_window_end:String,
    hsa_num: i32,

}


fn main(){
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .build().expect("failed to build client");

    let counties = vec!["81"];
    for county in &counties{
        download_remote_file(&client, county);
        let d = download2(county);
        let cities = d.integrated_county_timeseries_external_data;
        for k in cities {
            println!("{:?}",  k.fips_code);
        }


    }

    

    //let json  = data_file_to_hashmap("81.json");
        //let county_data = json.get("integrated_county_timeseries_external_data").expect("file should have FirstName key");
        //let test = inner_json.get("percent_positive_7_day").expect("Key needed for positive");
        //println!("{}", test)

}

fn download_remote_file(c: &reqwest::blocking::Client, county: &str) {
    let fname = format!("{}.json", county);
    println!(" working on file data");
    let remote_file =  format!("https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_timeseries_fips_370{}_external",county);
    let content = c.get(remote_file).send().expect("failed to get uri").bytes().expect("failed to get bytes");
    let mut f = File::create(&fname).expect("failed to create fileß");
    f.write_all(&content).expect("failed to write to file");
}

fn data_file_to_hashmap(fpath: &str) -> L1  {
    let data_file_as_string = fs::read_to_string(fpath).expect("Unable to read file");
    let data: L1 = serde_json::from_str(&data_file_as_string).unwrap();
    return data;
}

fn download2(county: &str) -> L1{
    let url = format!("https://covid.cdc.gov/covid-data-tracker/COVIDData/getAjaxData?id=integrated_county_timeseries_fips_370{}_external",county);
    let client = reqwest::blocking::Client::new();

    let resp = client
    .get(url)
    .header("Accept", "application/json")
    .send().expect("Error retrievinginformation from API")
    .json::<L1>()
    .expect("Error retrieving info.");

    return resp;

} 

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}