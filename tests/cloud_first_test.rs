use std::fmt::Display;

use sqlx::Connection;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{core::client::docker_client_instance, runners::AsyncRunner, ImageExt},
};

#[tokio::test]
async fn testcontainers_cloud_docker_engine() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let client = docker_client_instance().await?;
    let info = client.info().await?;

    let contains_cloud =
        matches!(info.server_version.as_ref(), Some(v) if v.contains("testcontainerscloud"));
    let contains_desktop =
        matches!(info.server_version.as_ref(), Some(v) if v.contains("Testcontainers Desktop"));

    if !(contains_cloud || contains_desktop) {
        Err(TestcontainersDesktopNotFound)?
    }

    let runtime = Some("Testcontainers Cloud")
        .filter(|_| contains_cloud)
        .or(info.operating_system.as_deref())
        .unwrap_or("unknown");
    let runtime = if contains_desktop {
        format!("{runtime} via Testcontainers Desktop app")
    } else {
        runtime.to_string()
    };

    println!(
        include_str!("./pretty_strings/tcc-congratulations.tmpl"),
        runtime = runtime
    );

    Ok(())
}

#[tokio::test]
// This test uses `AsyncRunner``, but the same is applicable for `SyncRunner`
// Also, it uses `GenericImage` from `testcontainers` for a whole picture, but you can use community modules from [`https://github.com/testcontainers/testcontainers-rs-modules-community`] instead
async fn create_postgres_client() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // configure and start Postgres instance
    let node = Postgres::default()
        .with_copy_to(
            "/docker-entrypoint-initdb.d/init.sql",
            include_bytes!("initdb.sql").to_vec(),
        )
        .start()
        .await?;

    // prepare connection string
    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        node.get_host_port_ipv4(5432).await?
    );
    // container is up, you can use it
    let mut conn = sqlx::postgres::PgConnection::connect(connection_string).await?;
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM guides")
        .fetch_one(&mut conn)
        .await?;
    assert_eq!(count, 7, "unexpected number of guides");
    Ok(())
}

#[derive(Debug)]
struct TestcontainersDesktopNotFound;

impl std::error::Error for TestcontainersDesktopNotFound {}

impl Display for TestcontainersDesktopNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, include_str!("./pretty_strings/tc-desktop-not-found.txt"))
    }
}
