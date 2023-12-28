# Instaclone Parseable
## Overview
A Service to save logs to Amazon S3 Bucket and view them in a web interface. A pretty cool stuff.

## How it works
It receives incoming logs from Vector through HTTP and saves them to Amazon S3 Bucket. 
The logs are then displayed in a web interface. 
The vector is installed on the machine that is running the service. 
In this case, the service is the <a href="">Instaclone RESTful API</a>.

## Deployment
The app is running on a docker container. You do not have to modify the Dockerfile.
The service is deployed to fly.io. You can view the logs at https://instaclone-logs.fly.dev.
If you want to deploy it to fly.io, check out their <a href="https://fly.io/docs/languages-and-frameworks/dockerfile/">documentation</a>.

## Getting Started
1. Get your AWS credentials and create an S3 bucket.
2. After that, set up any secret variables required by .env.example in a .env file in the root directory of the project.
3. You can run the app locally by running `docker-compose up -d` in the root directory of the project.

## More info
The P_USERNAME and P_PASSWORD are the registered credentials for the web interface. 
You can use it to log in and view the logs and also to send logs to the service. This is normally done by using Vector.
The vector configuration for this is in the vector.toml file in the api directory of the Instaclone project.

## More about Parseable
Documentation: https://www.parseable.io/docs/
GitHub Repository: https://github.com/parseablehq/parseable

## Get in touch
If you have any questions, you can reach me on Twitter: <a href="https://twitter.com/novqigarrix">@novqigarrix</a>