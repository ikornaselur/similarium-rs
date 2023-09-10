# Similarium
A Rust port of [Similarium](https://github.com/ikornaselur/similarium)

## Tests
The project is pretty lacking in terms of tests.. but hey, it's a side project!

# Deployment
## Env variables
Env variables expected:

* SLACK_CLIENT_ID: The Slack application client ID
* SLACK_CLIENT_SECRET: The slack application secret
* DATABASE_URL: A PostgrSQL connection string
* PORT: (default: 8080) Port for the API
* HOST: (default: 127.0.0.1) Host for the API
* WORKER_COUNT: (default: 3) How many background workers to run per process
* WORKER_MAX_POOL_SIZE: (default: 3) The worker connection pool size
