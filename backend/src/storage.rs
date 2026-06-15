use s3::{Bucket, BucketConfiguration, Region, creds::Credentials};

#[derive(Clone)]
pub struct Storage {
    bucket: Box<Bucket>,
}

impl Storage {
    pub async fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = std::env::var("S3_ENDPOINT")?;
        let region_name = std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string());
        let bucket_name = std::env::var("S3_BUCKET")?;
        let access_key = std::env::var("S3_ACCESS_KEY")?;
        let secret_key = std::env::var("S3_SECRET_KEY")?;

        let region = Region::Custom {
            region: region_name,
            endpoint,
        };
        let credentials =
            Credentials::new(Some(&access_key), Some(&secret_key), None, None, None)?;

        let exists = Bucket::new(&bucket_name, region.clone(), credentials.clone())?
            .with_path_style()
            .exists()
            .await?;

        if !exists {
            Bucket::create_with_path_style(
                &bucket_name,
                region.clone(),
                credentials.clone(),
                BucketConfiguration::default(),
            )
            .await?;
        }

        let bucket = Bucket::new(&bucket_name, region, credentials)?.with_path_style();

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
}
