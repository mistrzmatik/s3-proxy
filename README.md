<div align="center">
  <h1>S3 proxy</h1>
  <p>
    <strong>Read proxy for S3 bucket. Useful when your free S3 provider do not support public bucket.</strong>
  </p>
  <p>

![GitHub License](https://img.shields.io/github/license/mistrzmatik/s3-proxy)

  </p>
</div>

## Image

```shell
docker pull ghcr.io/mistrzmatik/s3-proxy:latest
```

## Configuration
| ENV         | Description                |
|-------------|----------------------------|
| REGION      | Region name (optional)     |
| ENDPOINT    | Region endpoint (required) |
| ACCESS_KEY  | Access key (required)      |
| SECRET_KEY  | Secret key (required)      |
| BUCKET_NAME | Bucket name (required)     |