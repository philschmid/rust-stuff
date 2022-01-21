# How to meter usage using `aws-rust-sdk` for AWS Makerplace Container solutions

> If you want to price your product based on number of tasks or pods used and have us meter that usage automatically, integrate with the `RegisterUsage` action.


## Resources

* [aws sdk overview](https://awslabs.github.io/aws-sdk-rust/)
* [crate: aws-sdk-marketplacemetering](https://crates.io/crates/aws-sdk-marketplacemetering)
* [documentation: aws-sdk-marketplacemetering](https://docs.rs/aws-sdk-marketplacemetering/latest/aws_sdk_marketplacemetering/)
* [AWS Marketplace Metering Service integration](https://docs.aws.amazon.com/marketplace/latest/userguide/entitlement-and-metering-for-paid-products.html)
* [RegisterUsage API](https://docs.aws.amazon.com/marketplacemetering/latest/APIReference/API_RegisterUsage.html)
AWS Marketplace Metering Service integration
* [Testing integration and preview mode for RegisterUsage](https://docs.aws.amazon.com/marketplace/latest/userguide/container-metering-registerusage.html#hourly-metering-preview-mode)


## How to test the integration

> To call RegisterUsage in preview mode, call RegisterUsage from the container image(s) by running your product on Amazon ECS or Amazon EKS with the AWS account you are using to list the product on AWS Marketplace. Your metering integration must dynamically set the AWS Region, rather than hard coding it, but when testing, launch at least one Amazon ECS task or Amazon EKS pod containing your paid container in the US East (N. Virginia) AWS Region, so that the AWS Marketplace operations team can verify your work with the logs in that Region.