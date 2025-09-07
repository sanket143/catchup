import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import lambda from "aws-cdk-lib/aws-lambda";
import apigw from "aws-cdk-lib/aws-apigateway";

export class AwsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const fn = new lambda.Function(this, "catchup", {
      code: lambda.Code.fromAsset("lib/lambda-handler"),
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: "index.handler",
    });

    const endpoint = new apigw.LambdaRestApi(this, "graphql", {
      handler: fn,
      restApiName: "HelloApi",
    });

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'AwsQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
