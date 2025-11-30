import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as httpapi from "aws-cdk-lib/aws-apigatewayv2";
import * as integrations from "aws-cdk-lib/aws-apigatewayv2-integrations";

export class AwsStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const fn = new lambda.Function(this, "catchup", {
      code: lambda.Code.fromAsset("../target/lambda/catchup/bootstrap.zip"),
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: "catchup",
      architecture: lambda.Architecture.ARM_64,
      environment: {
        DATABASE_URL: "",
      },
    });

    const httpApi = new httpapi.HttpApi(this, "CatchupHttpApi", {
      corsPreflight: {
        allowOrigins: [
          "https://sanket143.me",
          "https://catchup.sanket143.me",
          "https://catchup-c1x.pages.dev",
        ],
        allowHeaders: ["Content-Type", "Authorization"],
        allowMethods: [
          httpapi.CorsHttpMethod.GET,
          httpapi.CorsHttpMethod.POST,
          httpapi.CorsHttpMethod.OPTIONS,
        ],
      },
    });

    httpApi.addRoutes({
      path: "/{proxy+}",
      methods: [httpapi.HttpMethod.ANY],
      integration: new integrations.HttpLambdaIntegration(
        "CatchupIntegration",
        fn,
      ),
    });
  }
}
