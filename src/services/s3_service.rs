use aws_sdk_s3::Client;
use aws_sdk_s3::types::ByteStream;
use dotenv::dotenv;
use std::env;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use aws_sdk_s3::types::SdkError;
use aws_sdk_s3::operation::delete_object::DeleteObjectError;

pub async fn upload_file_to_s3(file_path: &str, key: &str, content_type: &str) -> Result<String, aws_sdk_s3::Error> {
    dotenv().ok();

    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set in .env");
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set in .env");
    let region = env::var("AWS_REGION").expect("AWS_REGION not set in .env");
    let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME not set in .env");

    let config = aws_sdk_s3::config::Builder::new()
        .region(aws_sdk_s3::Region::new(region))
        .credentials_provider(aws_sdk_s3::Credentials::new(aws_access_key_id, aws_secret_access_key, None, None, "env"))
        .build();

    let client = Client::from_conf(config);

    let mut file = File::open(file_path).await.expect("File not found");
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).await.expect("Failed to read file");

    let byte_stream = ByteStream::from(buffer);
    let result = client.put_object()
        .bucket(&bucket_name)
        .key(key)
        .body(byte_stream)
        .content_type(content_type)
        .send()
        .await?;

    let location = format!("https://{}.s3.{}.amazonaws.com/{}", bucket_name, region, key);
    Ok(location)
}

pub async fn delete_file_from_s3(key: &str) -> Result<(), SdkError<DeleteObjectError>> {
    dotenv().ok();

    let aws_access_key_id = env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not set in .env");
    let aws_secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not set in .env");
    let region = env::var("AWS_REGION").expect("AWS_REGION not set in .env");
    let bucket_name = env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME not set in .env");

    let config = aws_sdk_s3::config::Builder::new()
        .region(aws_sdk_s3::Region::new(region))
        .credentials_provider(aws_sdk_s3::Credentials::new(aws_access_key_id, aws_secret_access_key, None, None, "env"))
        .build();

    let client = Client::from_conf(config);
    client.delete_object()
        .bucket(&bucket_name)
        .key(key)
        .send()
        .await?;

    Ok(())
}
