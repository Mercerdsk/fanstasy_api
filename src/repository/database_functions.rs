use tokio_postgres::{NoTls,Config,Client, Connection};

pub async fn db_connection()->Result<Client,Box<dyn std::error::Error>>{
    let mut config = Config::new();
    config.host("192.168.10.211");
    config.user("postgres");
    config.password("GiChn32*");
    config.dbname("RoanuzFantasy");

    // Establish a connection
    let (_client, connection) = config.connect(NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(_client)
}