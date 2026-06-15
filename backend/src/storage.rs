use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};

use crate::config::S3Config;

#[derive(Clone)]
pub struct Storage {
    bucket: Box<Bucket>,
}

impl Storage {
    pub async fn connect(config: &S3Config) -> Result<Self, Box<dyn std::error::Error>> {
        let region = Region::Custom {
            region: config.region.clone(),
            endpoint: config.endpoint.clone(),
        };
        let credentials = Credentials::new(
            Some(&config.access_key),
            Some(&config.secret_key),
            None,
            None,
            None,
        )?;

        let exists = Bucket::new(&config.bucket, region.clone(), credentials.clone())?
            .with_path_style()
            .exists()
            .await?;

        if !exists {
            Bucket::create_with_path_style(
                &config.bucket,
                region.clone(),
                credentials.clone(),
                BucketConfiguration::default(),
            )
            .await?;
        }

        let bucket = Bucket::new(&config.bucket, region, credentials)?.with_path_style();

        Ok(Self { bucket })
    }

    pub async fn put(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.bucket
            .put_object_with_content_type(key, data, content_type)
            .await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let response = self.bucket.get_object(key).await?;
        Ok(response.into_bytes().to_vec())
    }

    pub async fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.bucket.delete_object(key).await?;
        Ok(())
    }
}
