use std::env;

use std::{thread, time};
use std::time::SystemTime;
//use std::result::Result;
//use std::io::Error;
//use std::future:Future;
use crate::futures::Future;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;

use tokio::runtime::Runtime;
extern crate futures;
extern crate tokio;

//use rusoto_credential::ProvideAwsCredentials;
use rusoto_core::credential::{AwsCredentials, StaticProvider};
use rusoto_core::{Region, RusotoError};
//use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{
   S3, S3Client, ListBucketsOutput,
};

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let runtime = Runtime::new().unwrap();
  let executor = runtime.executor();

  let region_name = "".to_string();
  let endpoint = "https://cellar-c2.services.clever-cloud.com".to_string();

  let region = Region::Custom {
    name: region_name.to_owned(),
    endpoint: endpoint.to_owned(),
  };

  let access_key = "blabal".to_string();
  let secret_key = "blibli".to_string();

  let client = S3Client::new_with(
    rusoto_core::request::HttpClient::new().expect("Failed to create HTTP client"),
    StaticProvider::from(AwsCredentials::new(access_key.clone(), secret_key.clone(), None, None)),
    region.clone(),
  );

  loop {
    let fut = S3::list_buckets(&client).then(|x| {
      match x {
        Ok(ListBucketsOutput { buckets, owner: _ }) => {
          match buckets {
            Some(_buckets) => println!("buckets"),
            None => println!("none")
          }
        }
        Err(RusotoError::Unknown(e)) => println!("{}", e.body_as_str()),
        Err(_error) => println!("yo")
      }
      Ok::<(),()>(())
    });

    executor.spawn(fut);

    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!("{}", datetime.format("%d/%m/%Y %T"));

    let ten_millis = time::Duration::from_millis(1000);
    thread::sleep(ten_millis);
  }
  //runtime.shutdown_on_idle().wait().unwrap();
}
