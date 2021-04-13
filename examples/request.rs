use failure::Fallible;
#[allow(unused_imports)]
use ftx_rs::{model, request, FtxClient};

async fn req<Q: request::Request + std::fmt::Debug>(c: &FtxClient, req: Q) {
    let s = format!("{:#?}", req);
    println!(
        "###\nrequest: {}\nresponse: {:#?}\n",
        s,
        c.request(req).await
    )
}

#[tokio::main]
async fn main() -> Fallible<()> {
    env_logger::init();

    let private_key: String = std::env::var("FTX_PRIVATE").unwrap();
    let public_key: String = std::env::var("FTX_PUBLIC").unwrap();

    let c = FtxClient::with_auth(&public_key, &private_key, None)?;

    req(&c, request::Markets).await;

    Ok(())
}
