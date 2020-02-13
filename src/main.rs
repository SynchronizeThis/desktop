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
   S3, S3Client, ListBucketsOutput, Bucket, ListObjectsV2Request, Object, ListObjectsV2Output,
   //ListObjectsV2Error,
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

  let access_key = "blabla".to_string();
  let secret_key = "blibli".to_string();

  let client = S3Client::new_with(
    rusoto_core::request::HttpClient::new().expect("Failed to create HTTP client"),
    StaticProvider::from(AwsCredentials::new(access_key.clone(), secret_key.clone(), None, None)),
    region.clone(),
  );

  loop {
    let fut = client.list_buckets().then(move |resp| {
      match resp {
        Ok(ListBucketsOutput { buckets, .. }) => {
          match buckets {
            Some(_buckets) => {
              let _buckets = _buckets.iter()
                .map(|bucket| {
                  println!("{:?}", bucket);
                  let list_obj_req = ListObjectsV2Request {
                    bucket: "apk".to_owned(),
                    start_after: Some("foo".to_owned()),
                    ..Default::default()
                  };
                  client.list_objects_v2(list_obj_req).then(move|resp| {
                    match resp {
                      Ok(ListObjectsV2Output { contents, .. }) => {
                        match contents {
                          Some(_objects) => {
                            let _objects = _objects.iter()
                              .map(|object| {
                                println!("{:?}", object);
                                object.clone()
                              })
                              .collect::<Vec<Object>>();
                          }
                          None => println!("none2")
                        }
                      }
                      //Err(ListObjectsV2Error) => println!("ListObjectsV2Error"),
                      Err(_error) => println!("yo")
                    }
                    Ok::<(),()>(())
                  });
                  bucket.clone()
                })
                .collect::<Vec<Bucket>>();
            }
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

    let ten_millis = time::Duration::from_millis(5000);
    thread::sleep(ten_millis);
  }
  //runtime.shutdown_on_idle().wait().unwrap();
}
