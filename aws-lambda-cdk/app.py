#!/usr/bin/env python3
import os

from aws_cdk import core as cdk

# For consistency with TypeScript code, `cdk` is the preferred import name for
# the CDK's core module.  The following line also imports it as `core` for use
# with examples from the CDK Developer's Guide, which are in the process of
# being updated to use `cdk`.  You may delete this import if you don't need it.
from aws_cdk import core

from infrastructure.rust_lambda import RustLambdaStack

# Environment
# CDK_DEFAULT_ACCOUNT and CDK_DEFAULT_REGION are set based on the
# AWS profile specified using the --profile option.
my_environment = cdk.Environment(account=os.environ["CDK_DEFAULT_ACCOUNT"], region=os.environ["CDK_DEFAULT_REGION"])


app = cdk.App()
rust_lambda = RustLambdaStack(app, "RustLambdaStack", env=my_environment)

app.synth()
