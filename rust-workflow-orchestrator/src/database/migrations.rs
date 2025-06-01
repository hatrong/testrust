// This file handles database migrations, including creating necessary tables.

use sqlx::sqlite::SqlitePool;
use sqlx::Error;

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), Error> {
    // Create the necessary tables for the workflow orchestration system
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS dags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            schedule TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dag_id INTEGER NOT NULL,
            task_id TEXT NOT NULL,
            command TEXT NOT NULL,
            FOREIGN KEY (dag_id) REFERENCES dags(id)
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS task_instances (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            execution_time DATETIME DEFAULT CURRENT_TIMESTAMP,
            state TEXT NOT NULL,
            FOREIGN KEY (task_id) REFERENCES tasks(id)
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}