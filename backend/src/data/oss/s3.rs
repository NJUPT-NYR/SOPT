use super::*;
use crate::error::{error_string, Error};
use aws_sdk_s3::Client;
use futures::{Future, TryFutureExt, TryStreamExt};
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};

pub struct AwsWrapper {
    pub client: Client,
}

impl ObjectStorageService for AwsWrapper {
    fn create_bucket<'a>(
        &self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
        Box::pin(
            self.client
                .create_bucket()
                .bucket(name)
                .send()
                .map_ok(|_| ())
                .map_err(|e| Error::OSSError(Box::new(e))),
        )
    }

    fn delete_bucket<'a>(
        &self,
        name: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
        Box::pin(
            self.client
                .delete_bucket()
                .bucket(name)
                .send()
                .map_ok(|_| ())
                .map_err(|e| Error::OSSError(Box::new(e))),
        )
    }

    fn put<'a>(
        &self,
        name: &'a str,
        key: &'a str,
        content: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
        use aws_smithy_http::byte_stream::ByteStream;
        Box::pin(
            self.client
                .put_object()
                .bucket(name)
                .key(key)
                .body(ByteStream::from(content.to_owned()))
                .send()
                .map_ok(|_| ())
                .map_err(|e| Error::OSSError(Box::new(e))),
        )
    }

    fn get<'a>(
        &'a self,
        name: &'a str,
        key: &'a str,  // object key
        dir: &'a Path, // local storage prefix path
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
        let future = async move {
            let mut data = self
                .client
                .get_object()
                .bucket(name)
                .key(key)
                .send()
                .await
                .map_err(|e| Error::OSSError(Box::new(e)))?
                .body;

            let file_path = dir.join(key);
            let parent_dir = file_path.parent().unwrap();
            if !parent_dir.exists() {
                create_dir_all(parent_dir).map_err(|e| error_string(e))?;
            }
            let file = File::create(&file_path).map_err(|e| error_string(e))?;
            let mut buf_writer = BufWriter::new(file);
            while let Some(bytes) = data.try_next().await.unwrap() {
                buf_writer.write(&bytes).map_err(|e| error_string(e))?;
            }
            buf_writer.flush().unwrap();
            Ok(())
        };
        Box::pin(future)
    }

    fn delete<'a>(
        &self,
        name: &'a str,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
        Box::pin(
            self.client
                .delete_object()
                .bucket(name)
                .key(key)
                .send()
                .map_ok(|_| ())
                .map_err(|e| Error::OSSError(Box::new(e))),
        )
    }

    fn head<'a>(
        &self,
        name: &'a str,
        key: &'a str,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'a>> {
        Box::pin(
            self.client
                .head_object()
                .bucket(name)
                .key(key)
                .send()
                .map_ok(|_| ())
                .map_err(|e| Error::OSSError(Box::new(e))),
        )
    }

    fn list_buckets(&self) -> Pin<Box<dyn Future<Output = Result<(), Error>>>> {
        Box::pin(
            self.client
                .list_buckets()
                .send()
                .map_ok(|_| ())
                .map_err(|e| Error::OSSError(Box::new(e))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_all() {
        dotenv().ok();

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
                .load()
                .await
        });
        let _s3 = Arc::new(s3::AwsWrapper {
            client: aws_sdk_s3::Client::new(&config),
        }) as Arc<dyn ObjectStorageService>;

        let bucket_name = "testbucket";
        match OSS.clone().create_bucket(bucket_name).await {
            Ok(_) => {
                OSS.clone()
                    .put(bucket_name, "test/1", &"test_content1".as_bytes())
                    .await
                    .unwrap();
                let _ = OSS
                    .clone()
                    .get(bucket_name, "test/1", Path::new("/tmp/bucket/"))
                    .await
                    .unwrap();
                let _ = OSS.clone().list_buckets().await.unwrap();
                OSS.clone().delete(bucket_name, "test/1").await.unwrap();
                OSS.clone().delete_bucket(bucket_name).await.unwrap();
                assert!(true)
            }
            Err(_) => assert!(false),
        }
    }
}
