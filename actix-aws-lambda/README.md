# Example on how to run `actix-web` using `AWS-Lambda`

the `is_running_on_lambda` checks wether the `AWS_LAMBDA_RUNTIME_API` is set if not it will start a normal `actix-web` server.


## Getting started

test locally

```bash
cargo run
```

deploy

```bash
cdk deploy
```

## Resources

* [Github](https://github.com/hanabu/lambda-web)
* [Documentation](https://docs.rs/lambda-web/0.1.6/lambda_web/)

