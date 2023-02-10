mod prisma;

use std::error::Error;

use prisma::PrismaClient;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = PrismaClient::_builder().build().await?;

    client
        .user()
        .create(
            Uuid::new_v4().to_string(),
            "marcus@radell.net".to_string(),
            vec![],
        )
        .exec()
        .await?;

    let rows = client.user().find_many(vec![]).exec().await?;

    println!("{rows:?}");

    Ok(())
}
