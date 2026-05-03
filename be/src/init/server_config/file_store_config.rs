#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStoreConfig {
    pub file_store_type: FileStoreType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileStoreType {
    Local(LocalFileStoreConfig),
    AwsS3(AwsS3Config),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalFileStoreConfig {
    pub local_file_store_base_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AwsS3Config {
    pub aws_s3_bucket_name: String,
    pub aws_s3_access_key: String,
    pub aws_s3_secret_key: String,
    pub aws_s3_region: String,
}

impl FileStoreConfig {
    /// Perform the `local` operation as implemented by this function.
    ///
    /// # Arguments
    /// * `local_file_store_base_path` -
    /// # Returns
    /// Returns the value produced by this function.
    pub fn local(local_file_store_base_path: String) -> Self {
        Self {
            file_store_type: FileStoreType::Local(LocalFileStoreConfig {
                local_file_store_base_path,
            }),
        }
    }

    pub fn aws_s3(
        aws_s3_bucket_name: String,
        aws_s3_access_key: String,
        aws_s3_secret_key: String,
        aws_s3_region: String,
    ) -> Self {
        Self {
            file_store_type: FileStoreType::AwsS3(AwsS3Config {
                aws_s3_bucket_name,
                aws_s3_access_key,
                aws_s3_secret_key,
                aws_s3_region,
            }),
        }
    }
}
