# Gitlab-Weeek webhook server.

## Develop.
- build project
  ```bash
  cargo build
  ```
- before tests, export example environs.
  ```bash
  source example-environs.bash
  ```
- going test.
  ```
  cargo test --test integration_test
  ```
- going test coverage.
  ```bash
  cargo llvm-cov --test integration_test
  ```
- look html test coverage.
  ```bash
  cargo llvm-cov --test integration_test --html
  open target/llvm-cov/html/index.html
  ```

## Start server.
- input example json template to gitlab.
- export self environs by example and run server.
  ```bash
  cargo run
  ```

## Articals.
- [Gitlab Webhook](https://docs.gitlab.com/ee/user/project/integrations/webhooks.html)
- [Mocking Rust with mockall](https://blog.logrocket.com/mocking-rust-mockall-alternatives/)
- [Create linux service](https://www.shubhamdipt.com/blog/how-to-create-a-systemd-service-in-linux/)
