use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello?name=Erich").await?.print().await?;

    hc.do_get("/hello/Erich").await?.print().await?;

    hc.do_get("/fallback.html").await?.print().await?;

    let login_req = hc.do_post(
        "/api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        }),
    );
    login_req.await?.print().await?;

    let login_req = hc.do_post(
        "/api/login",
        json!({
            "username": "bad-username",
            "pwd": "wrong-pwd"
        }),
    );
    login_req.await?.print().await?;

    let ticket_req = hc.do_post(
        "/api/tickets",
        json!({
            "title":"test ticket"
        }),
    );
    ticket_req.await?.print().await?;

    let ticket_req = hc.do_get("/api/tickets");
    ticket_req.await?.print().await?;

    Ok(())
}
