use std::env;

use std::time::SystemTime;
//use std::{thread, time};
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
    Bucket,
    ListBucketsOutput,
    ListObjectsV2Output,
    //ListObjectsV2Error,
    ListObjectsV2Request,
    //Object,
    S3Client,
    S3,
};

fn get_objects_from_bucket(client: S3Client, bucket: Bucket) {
    let list_obj_req = ListObjectsV2Request {
        bucket: bucket.name.unwrap(),
        ..Default::default()
    };

    tokio::spawn(client.list_objects_v2(list_obj_req).then(|resp| {
        println!("{:#?}", resp);
        match resp {
            Ok(ListObjectsV2Output { contents, .. }) => {
                match contents {
                    Some(_objects) => {
                        println!("{:#?}", _objects);
                        let _objects = _objects
                            .iter()
                            .for_each(|object| {
                                println!("{:?}", object);
                                //object.clone();
                            });
                    }
                    None => println!("none2"),
                }
            }
            //Err(ListObjectsV2Error) => println!("ListObjectsV2Error"),
            Err(_error) => println!("yo"),
        }
        Ok::<(), ()>(())
    }));
}

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

    let access_key = "fake".to_string();
    let secret_key = "fake".to_string();

    let client = S3Client::new_with(
        rusoto_core::request::HttpClient::new().expect("Failed to create HTTP client"),
        StaticProvider::from(AwsCredentials::new(
            access_key.clone(),
            secret_key.clone(),
            None,
            None,
        )),
        region.clone(),
    );


    let fut = client.list_buckets().then(move |resp| {
        match resp {
            Ok(ListBucketsOutput { buckets, .. }) => {
                match buckets {
                    Some(_buckets) => {
                        println!("Buckets number: {:#?}", _buckets.len());
                        _buckets
                            .iter()
                            .for_each(|bucket| {
                                //println!("{:#?}", bucket);
                                get_objects_from_bucket(client.to_owned(), bucket.to_owned());
                            });
                    }
                    None => println!("none"),
                }
            }
            Err(RusotoError::Unknown(e)) => println!("{}", e.body_as_str()),
            Err(_error) => println!("yo"),
        }
        Ok::<(), ()>(())
    });

    executor.spawn(fut);

    let system_time = SystemTime::now();
    let datetime: DateTime<Utc> = system_time.into();
    println!("{}", datetime.format("%d/%m/%Y %T"));

    //let ten_millis = time::Duration::from_millis(5000);
    //thread::sleep(ten_millis);;

    runtime.shutdown_on_idle().wait().unwrap();
}
