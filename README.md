# testcontainers-cloud-rs-example

This is an example repository with a simple test confirming proper connection from Testcontainers Desktop (or the CI agent) to your [Testcontainers Cloud](https://app.testcontainers.cloud) account.

For details on how to bootstrap Testcontainers in an actual project, please refer to the [Testcontainers Rust Quickstart](https://rust.testcontainers.org/quickstart/testcontainers).

## Clone the repository and run the first Testcontainers test suite

```shell
git clone https://github.com/AtomicJar/testcontainers-cloud-rs-example.git
cd testcontainers-cloud-rs-example
make test
```

The `Make` command will run the test suite using `cargo test -- --nocapture`.

Note that it's important to add the `--nocapture` flag, otherwise the output of the tests will be suppressed.

## Confirm your environment is configured correctly

The test output should show the Testcontainers logo and which container runtime was used:  

```shell
running 2 tests
        ████████╗███████╗███████╗████████╗ ██████╗ ██████╗ ███╗   ██╗████████╗ █████╗ ██╗███╗   ██╗███████╗██████╗ ███████╗ 
        ╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██╔═══██╗████╗  ██║╚══██╔══╝██╔══██╗██║████╗  ██║██╔════╝██╔══██╗██╔════╝ 
           ██║   █████╗  ███████╗   ██║   ██║     ██║   ██║██╔██╗ ██║   ██║   ███████║██║██╔██╗ ██║█████╗  ██████╔╝███████╗ 
           ██║   ██╔══╝  ╚════██║   ██║   ██║     ██║   ██║██║╚██╗██║   ██║   ██╔══██║██║██║╚██╗██║██╔══╝  ██╔══██╗╚════██║ 
           ██║   ███████╗███████║   ██║   ╚██████╗╚██████╔╝██║ ╚████║   ██║   ██║  ██║██║██║ ╚████║███████╗██║  ██║███████║ 
           ╚═╝   ╚══════╝╚══════╝   ╚═╝    ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝╚══════╝

  Congratulations on running your first test! 🎉
  Runtime used:
      Testcontainers Cloud via Testcontainers Desktop app

  You can now return to the website to complete your onboarding.

test testcontainers_cloud_docker_engine ... ok
test create_postgres_client ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.04s
```

## (optional) Use Testcontainers Desktop to easily debug the database

[Testcontainers Desktop](https://testcontainers.com/desktop/) helps developers with common tasks such as debugging your Testcontainers-powered dependencies. Let's practice!

The tests in this project create a PostgreSQL database and populate it with sample data. You can [set a fixed port](https://newsletter.testcontainers.com/announcements/set-fixed-ports-to-easily-debug-development-services) for the `postgres` service, then [freeze containers shutdown](https://newsletter.testcontainers.com/announcements/freeze-containers-to-prevent-their-shutdown-while-you-debug) to easily connect to the database from your IDE after your tests run.

See if you can inspect the database. Username: `postgres`. Password: `postgres`.
 
