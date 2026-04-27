pub struct FileStoreConfig {
    pub file_store_type: FileStoreType,
}

pub enum FileStoreType {
    Local(LocalFileStoreConfig),
    AwsS3(AwsS3Config),
}

pub struct LocalFileStoreConfig {
    pub local_file_store_base_path: String,
}

pub struct AwsS3Config {
    pub aws_s3_bucket_name: String,
    pub aws_s3_access_key: String,
    pub aws_s3_secret_key: String,
    pub aws_s3_region: String,
}
