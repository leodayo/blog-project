use chrono::offset;
use sqlx::PgPool;

use crate::domain::{error::AppError, post::Post};

pub struct PostRepository {
    pool: PgPool,
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_post(
        &self,
        author_id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, AppError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, content, author_id)
            VALUES ($1, $2, $3)
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            title,
            content,
            author_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Db)?;

        Ok(post)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Post, AppError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            WHERE posts.id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                return AppError::PostNotFound(id);
            }
            AppError::Db(e)
        })?;

        Ok(post)
    }

    // TODO() Reconsider the approach.
    //
    // I really don't like current implementation
    // OFFSET is infamous for being terrible performance-wise.
    // Another approach should be investigated and implemented.
    // What I can think of from top of my head are those:
    //   1. Maybe instead of OFFSET do WHERE id > $offset
    //   2. Move to cursor-based approach
    //   3. Use limit+1 approach (i.e. extracting one extra record)
    //      over the limit, and if it exists then return 'limit'
    //      amount of records and a flag "has_next/has_more"
    //
    // Each of the options above have it's own drawback, hence why I didn't
    // choose one myself and didn't just implement it right away.
    // I would appreceate any suggestions/thoughts on the topic.
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<(Vec<Post>, i64), AppError> {
        let posts: Vec<Post> = sqlx::query_as!(
            Post,
            r#"
            SELECT id, title, content, author_id, created_at, updated_at
            FROM posts
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Db)?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM posts
            "#
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::Db)?
        .unwrap_or(0);

        Ok((posts, total))
    }

    pub async fn update_post(&self, id: i64, title: &str, content: &str) -> Result<Post, AppError> {
        let post = sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $1, content = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, title, content, author_id, created_at, updated_at
            "#,
            title,
            content,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                return AppError::PostNotFound(id);
            }
            AppError::Db(e)
        })?;

        Ok(post)
    }

    pub async fn delete_post(&self, id: i64) -> Result<(), AppError> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM posts WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::Db)?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::PostNotFound(id));
        }

        Ok(())
    }
}
