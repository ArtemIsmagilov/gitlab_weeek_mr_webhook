# Gitlab-Weeek webhook server

## Develop

- build project

  ```bash
  cargo build
  ```

- before tests, export example environs.

  ```bash
  set -a && . example.env && set +a
  ```

- going test.

  ```bash
  cargo test
  ```

- going test coverage.

  ```bash
  cargo install cargo-llvm-cov
  cargo llvm-cov
  ```

- look html test coverage.

  ```bash
  cargo llvm-cov --html
  open target/llvm-cov/html/index.html
  ```

## Start server

- input example json template to gitlab.
- export self environs by example and run server.

  ```bash
  cargo run
  ```

## Articals

- [Gitlab Webhook](https://docs.gitlab.com/ee/user/project/integrations/webhooks.html)
- [Mocking Rust with mockall](https://blog.logrocket.com/mocking-rust-mockall-alternatives/)
- [Create linux service](https://www.shubhamdipt.com/blog/how-to-create-a-systemd-service-in-linux/)
- [Nginx location](https://server-gu.ru/nginx-location/)
- [Simple deploy Gunicorn](https://docs.gunicorn.org/en/latest/deploy.html)
- [Docker compose](https://docs.docker.com/reference/compose-file/)
