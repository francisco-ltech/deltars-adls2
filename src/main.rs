extern crate anyhow;
extern crate deltalake;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // The following ENV_VARS must be set:
    // E.g.: export AZURE_STORAGE_ACCOUNT_NAME=
    // E.g.: export AZURE_STORAGE_ACCOUNT_KEY=

    let account = std::env::var("AZURE_STORAGE_ACCOUNT_NAME").unwrap();
    let table_uri = format!("adls2://{}/{}/{}/delta-0.8.0/", account, "sandbox", "francisco");
    
    let table = deltalake::open_table(table_uri.as_str())
            .await
            .unwrap();

    println!("{}", table);
    
    Ok(())
}