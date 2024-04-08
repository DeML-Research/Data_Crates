// minimal example with ballista

use ballista::prelude::{BallistaConfig, BallistaContext, Result};
use datafusion::execution::options::CsvReadOptions;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {

    // Config Ballista Session
    let ballista_config = BallistaConfig::builder()
        .set("ballista.shuffle.partitions", "1")
        .build()?;
    
    // Create a new session
    let ctx_session = BallistaContext::standalone(&ballista_config, 2).await?;

    // Read data into the created session, from a csv
    let csv_file_path = "../data/uniswapv3_data.csv";
    ctx_session.register_csv("uniswapv3", csv_file_path, CsvReadOptions::new().has_header(true))
                    .await?;

    // Useful and basic operations in sql
    
    // 0: Show the current session config options
    // let sql_0 = ctx_session
    //    .sql("SELECT * FROM information_schema.tables")
    //    .await?;

    // 1: Get the columns names
    let sql_1 = ctx_session
        .sql("show columns from uniswapv3")
        .await?;

    // An example of a filter operation:
    // Using SQL: 
    // let sql_filtered_data = ctx_session
    //    .sql("SELECT Token0, Token1
    //          FROM uniswapv3
    //          WHERE VolumeUSD >= 1000
    //       ")
    //    .await?;
    
    // Collect the results 
    let resulted_data = sql_1.collect().await?;

    // Visualize results
    for batch in &resulted_data {
        println!("{:?}", batch);
    }

    Ok(())

}

