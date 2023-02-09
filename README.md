# Self Service

This is a chassis for HTTP services. It aims to get you up and running with features expected on more opinionated web frameworks while you can keep control of the specific pieces.

## Features checklist

### Ready

- Logs
  - tracing + opentelemetry
- Health check
  - Simple `/health` endpoint
- Trace Context extraction
  - `axum_tracing_opentelemetry` and `opentelemetry` propagators did it.
- Config through e.g. yaml or TOML and env vars override
  - Using `config` for that
- Configured HTTP client
  - request and response tracing
  - Trace Context propagation

### TODO

- Evaluate addition of the `http_problem` crate, or something similar
- Add endpoint to print information similar to [actuator info](https://docs.spring.io/spring-boot/docs/current/actuator-api/htmlsingle/#info)
- Evaluate crates that generate OpenAPI schemas
