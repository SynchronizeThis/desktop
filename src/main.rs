use std::env;
//use std::error::Error;

use futures::future::Future;
use s3::serde_types::Object;
//use futures::future::IntoFuture;

use tokio::runtime::Runtime;

use s3::bucket::Bucket;
use s3::region::Region;
use s3::credentials::Credentials;
use s3::error::S3Error;

fn get_bucket_list(
  bucket: &s3::bucket::Bucket,
  path: &str,
) -> Box<dyn Future<Item = Vec<Object>, Error = S3Error> + Send> {
  let fut = bucket.list_all_async(path.to_string(), Some("/".to_string()))
    .map(|res| {
      res.into_iter()
        .flat_map(|a| a.contents)
        .collect()
    });
  Box::new(fut)
}

fn main() {
  let runtime = Runtime::new().unwrap();
  let executor = runtime.executor();

  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  let bucket_name = "apk";
  let region_name = "".to_string();
  let endpoint = "https://cellar-c2.services.clever-cloud.com".to_string();
  let region = Region::Custom { region: region_name, endpoint };

  let access_key = String::from("blabla");
  let secret_key = String::from("blibli");
  let credentials = Credentials::new(Some(access_key), Some(secret_key), None, None);

  let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

  let f = get_bucket_list(&bucket, &"".to_string()).map(|data| {
    data.into_iter().map(|a| println!("{} {}, {} - {}", a.last_modified, a.key, a.e_tag, a.size)).collect()
  }).map_err(|e| {
    println!("{:?}", e);
  });

  executor.spawn(f);
  runtime.shutdown_on_idle().wait().unwrap();
}
