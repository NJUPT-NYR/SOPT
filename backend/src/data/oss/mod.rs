mod s3;

use crate::{config::CONFIG, error::Error};
use aws_config::timeout::TimeoutConfig;
use aws_sdk_s3::config::{Credentials, Region};
use futures::Future;
use lazy_static::lazy_static;
use std::{path::Path, pin::Pin, sync::Arc, time::Duration};

lazy_static! {
    pub static ref OSS: Arc<dyn ObjectStorageService> = {
        use futures::executor::block_on;
        let config = block_on(async {
            aws_config::from_env()
                .region(Region::new(CONFIG.oss.region.clone()))
                .endpoint_url(CONFIG.oss.endpoint.clone())
                .credentials_provider(Credentials::new(
                    CONFIG.oss.access_key.clone(),
                    CONFIG.oss.secret_key.clone(),
                    None,
                    None,
                    "from_env",
                ))
                .timeout_config(
                    TimeoutConfig::builder()
                        .operation_timeout(Duration::from_secs(30))
                        .operation_attempt_timeout(Duration::from_secs(10))
                        .connect_timeout(Duration::from_secs(3))
                        .build(),
                )
                .load()
                .await
        });
        Arc::new(s3::AwsWrapper {
            client: aws_sdk_s3::Client::new(&config),
        }) as Arc<dyn ObjectStorageService>
    };
}

pub trait ObjectStorageService: Send + Sync {
    fn create_bucket<'a>(
        &self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>;
    // Result<(), Error>;
    fn delete_bucket<'a>(
        &self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>;
    fn put<'a>(
        &self,
        name: &'a str,
        key: &'a str,
        content: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>;
    fn get<'a>(
        &'a self,
        name: &'a str,
        path: &'a str,
        dir: &'a Path,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>;
    fn delete<'a>(
        &self,
        name: &'a str,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>;
    fn head<'a>(
        &self,
        name: &'a str,
        path: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>>;
    fn list_buckets(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>>>>;
}
