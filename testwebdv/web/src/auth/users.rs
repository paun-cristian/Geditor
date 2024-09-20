use sqlx::mysql::MySqlPool;
use tracing::info;

pub async fn save_user(user: &super::User) -> Result<(), sqlx::Error> {
        let query = r#"
            INSERT INTO users (id, name, email, password)
            VALUES (?, ?, ?, ?)
        "#;
        let pool = MySqlPool::connect("mysql://root:Thefilthycunt777@localhost/mydb")
        .await
        .unwrap();
        info!("Connected to MySQL");

        sqlx::query(query)
            .bind(&user.id)
            .bind(&user.name)
            .bind(&user.email)
            .bind(&user.password)
            .execute(&pool)
            .await?;

        info!("User {} saved", user.id);
        Ok(())
    }
