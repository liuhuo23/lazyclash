use std::path::{Path, PathBuf};

use crate::config::get_data_dir;
use crate::prfitem::PrfItem;
use chrono::Utc;
use color_eyre::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, FromRow, Pool, Row, Sqlite};
use tracing::debug;
use tracing_subscriber::fmt::format;

pub fn get_db_file() -> Result<PathBuf> {
    let mut db_file = get_data_dir();
    if !db_file.exists() {
        std::fs::create_dir_all(&db_file)?;
    }
    db_file.push("lazyclash.db");
    debug!("db_file: {}", db_file.display());
    Ok(db_file)
}

pub async fn init() -> Result<()> {
    init_db(get_db_file()?).await?;
    Ok(())
}

async fn init_db(db_file: PathBuf) -> Result<()> {
    let db_url = format!("sqlite://{}", db_file.display());
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }
    let poll = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", db_file.display()))
        .await?;
    // 运行迁移
    let migration = r#"
        CREATE TABLE IF NOT EXISTS prf_items (
            uid TEXT,
            itype TEXT,
            name TEXT,
            file TEXT,
            desc TEXT,
            url TEXT,
            selected INTEGER, -- 在 SQLite 中，布尔类型被存储为 INTEGER (0 或 1)
            extra TEXT, -- 使用 TEXT 类型存储 JSON 数据
            updated INTEGER,
            home TEXT,
            file_data TEXT
        );
    "#;
    sqlx::query(migration).execute(&poll).await?;
    Ok(())
}

pub async fn insert_prf_item(item: &PrfItem) -> Result<i64> {
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:///{}", get_db_file()?.display()))
        .await?;
    let now = Utc::now();
    // 将 PrfExtra 序列化为 JSON 字符串
    let extra_json = item
        .extra
        .as_ref()
        .map(|extra| serde_json::to_string(extra).unwrap_or_default());

    // 构建插入语句
    let mut query = sqlx::query("INSERT INTO prf_items (uid, itype, name, file, desc, url, selected, extra, updated, home, file_data) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)");

    query = query
        .bind(item.uid.as_deref())
        .bind(item.itype.as_deref())
        .bind(item.name.as_deref())
        .bind(item.file.as_deref())
        .bind(item.desc.as_deref())
        .bind(item.url.as_deref())
        .bind(item.selected.map(|b| b as i32)) // 在 SQLite 中，布尔值被存储为 1 或 0
        .bind(extra_json.as_deref())
        .bind(now.timestamp())
        .bind(item.home.as_deref())
        .bind(item.file_data.as_deref());

    // 执行插入操作并返回插入的行的 ID
    let row_id = query.execute(&pool).await?.last_insert_rowid();

    Ok(row_id)
}

pub async fn query_prf_item() -> Result<Vec<PrfItem>> {
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:///{}", get_db_file()?.display()))
        .await?;

    let items = sqlx::query_as::<_, PrfItem>(
        "SELECT uid, itype, name, file, desc, url, selected, extra, updated, home, file_data FROM prf_items",
    )
   .fetch_all(&pool)
   .await?;

    Ok(items)
}

impl FromRow<'_, sqlx::sqlite::SqliteRow> for PrfItem {
    fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(PrfItem {
            uid: Some(row.try_get("uid")?),
            itype: row.try_get("itype")?,
            name: row.try_get("name")?,
            file: row.try_get("file")?,
            desc: row.try_get("desc")?,
            url: row.try_get("url")?,
            selected: row.try_get("selected")?,
            extra: None,
            updated: None,
            home: row.try_get("home")?,
            file_data: row.try_get("file_data")?,
        })
    }
}
