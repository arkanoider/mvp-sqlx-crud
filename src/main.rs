use sqlx::FromRow;
use sqlx_crud::SqlxCrud;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, FromRow, SqlxCrud)]
pub struct TestDispute {
    pub id: Uuid,
    pub order_id: Uuid,
}

impl TestDispute {
    pub fn new(id: Uuid, order_id: Uuid) -> Self {
        Self { id, order_id }
    }
}

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod tests {
    use crate::TestDispute;
    use anyhow::Result;
    use sqlx::SqlitePool;
    use sqlx_crud::Crud;

    #[tokio::test]
    async fn test_new_dispute_fake() -> Result<()> {
        let db_url = "sqlite://mostro.db";

        let _poolaaa = SqlitePool::connect(&db_url).await?;

        let a = uuid::Uuid::new_v4();
        let b = uuid::Uuid::new_v4();
        let dispute = TestDispute::new(a, b);
        println!("{:?}", dispute);
        dispute.create(&_poolaaa).await?;

        Ok(())
    }
}
