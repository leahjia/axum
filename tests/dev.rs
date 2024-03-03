#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello").await?.print().await?;
    hc.do_get("/hello?name=Jane").await?.print().await?;
    hc.do_get("/helloagain/Mike").await?.print().await?;

    Ok(())
}
