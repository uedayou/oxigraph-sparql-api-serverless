# oxigraph-sparql-api-serverless

This is the rust program which can deploy it as Web API of SPARQL Endpoint using [Oxigraph](https://github.com/oxigraph/oxigraph) on AWS Lambda and API Gateway.

## Build using Docker

```
$ cp YOUR_TURTLE_FILE_PATH rdf/dump.ttl 
$ docker image build -t oxigraph-build -f Dockerfile.build .
$ docker container run --rm -v $PWD:/code -v $HOME/.cargo/registry:/root/.cargo/registry -v $HOME/.cargo/git:/root/.cargo/git oxigraph-build
```

## Deploy on AWS using SAM CLI

```
$ sam package --template-file template.yaml --output-template-file packaged.yaml --s3-bucket S3_BUCKET_NAME
$ sam deploy --template-file packaged.yaml --stack-name PROJECT_NAME --capabilities CAPABILITY_IAM
```

## How to use as Web API

```
https://${ID}.execute-api.${Region}.amazonaws.com/Prod/sparql?query={sparql query with URL-encoded}&format={json(default) or xml}
```
