use rusoto_core::Region;
use rusoto_s3::{
    S3, S3Client, Delete, Bucket, Object,
    CreateBucketRequest, DeleteBucketRequest,
    PutObjectRequest, DeleteObjectRequest,
};
pub struct S3Bucket {
    pub s3: S3Client,
    pub bucket: String,
    pub region: Region,
}

impl S3Bucket {

    pub async fn new(bucket: String) -> Self {
        let region = Region::UsWest2;
        let s3 = S3Client::new(region.clone());
        Self { s3, region, bucket }
    }

    pub async fn new_bucket(self, name: String) -> anyhow::Result<()> {
        self.s3.create_bucket(CreateBucketRequest { bucket: name.clone(), ..Default::default() })
            .await?;
        Ok(())
    }

    pub async fn delete_bucket(self, name: String) -> anyhow::Result<()> {
        self.s3.delete_bucket(DeleteBucketRequest { bucket: name.clone(), ..Default::default() })
            .await?;
        Ok(())
    }

    pub async fn cleanup(self) -> anyhow::Result<()> {
        self.s3.delete_bucket(DeleteBucketRequest { bucket: self.bucket.clone(), ..Default::default() })
            .await?;
        Ok(())
    }

    pub async fn put_object(self, key: String, _loc: String) -> anyhow::Result<()> {
        let mut obj: Vec<u8> = Vec::new();
        self.s3.put_object(PutObjectRequest {
            bucket: self.bucket,
            key: key.into(),
            body: Some(obj.into()),
            ..Default::default()
        })
            .await?;
        Ok(())
    }

    pub async fn delete_object(self, key: String) -> anyhow::Result<()> {
        self.s3.delete_object(DeleteObjectRequest { bucket: self.bucket.clone(), key, ..Default::default() })
            .await?;
        Ok(())
    }

}
