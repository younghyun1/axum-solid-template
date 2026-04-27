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
    aws_s3_bucket_name: String,
    aws_s3_access_key: String,
    aws_s3_secret_key: String,
    aws_s3_region: String,
}
