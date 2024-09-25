use sqlx::Connection;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{runners::AsyncRunner, ImageExt},
};

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
