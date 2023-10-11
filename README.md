# chowkidar

AWS Cloudwatch -> Discord webhook handler

Supports only two states:

- Alarm
- OK

Sends Discord embedded webhook message when state changes.

## Build

1. Install Rust and `cargo-lambda`: https://www.cargo-lambda.info/guide/getting-started.html
2. Run the following command to generate the output zip:

```bash
cargo lambda build --release --output-format zip
```

And then upload the output zip to the Lambda function.
