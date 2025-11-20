use crate::utils::{AppError, AppResult};
use casbin::{CoreApi, Enforcer, Filter};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct CasbinRule {
    pub id: Option<i32>,
    pub ptype: String,
    pub v0: Option<String>,
    pub v1: Option<String>,
    pub v2: Option<String>,
    pub v3: Option<String>,
    pub v4: Option<String>,
    pub v5: Option<String>,
}

pub struct CasbinService {
    enforcer: Arc<RwLock<Enforcer>>,
    pool: PgPool,
}

impl CasbinService {
    pub async fn new(pool: PgPool, model_path: &str) -> AppResult<Self> {
        // Create enforcer with database adapter
        let adapter = casbin_sqlx_adapter::SAdapter::new(pool.clone()).await?;
        let enforcer = Enforcer::new(model_path, adapter).await?;
        
        let service = CasbinService {
            enforcer: Arc::new(RwLock::new(enforcer)),
            pool,
        };

        Ok(service)
    }

    pub async fn enforce(&self, sub: &str, obj: &str, act: &str) -> AppResult<bool> {
        let enforcer = self.enforcer.read().await;
        let result = enforcer.enforce((sub, obj, act))?;
        Ok(result)
    }

    pub async fn add_policy(&self, sub: &str, obj: &str, act: &str) -> AppResult<bool> {
        let mut enforcer = self.enforcer.write().await;
        let result = enforcer.add_policy(vec![sub.to_string(), obj.to_string(), act.to_string()]).await?;
        Ok(result)
    }

    pub async fn remove_policy(&self, sub: &str, obj: &str, act: &str) -> AppResult<bool> {
        let mut enforcer = self.enforcer.write().await;
        let result = enforcer.remove_policy(vec![sub.to_string(), obj.to_string(), act.to_string()]).await?;
        Ok(result)
    }

    pub async fn get_all_policies(&self) -> AppResult<Vec<Vec<String>>> {
        let enforcer = self.enforcer.read().await;
        let policies = enforcer.get_policy();
        Ok(policies)
    }

    pub async fn update_policy(
        &self,
        old_rule: Vec<String>,
        new_rule: Vec<String>,
    ) -> AppResult<bool> {
        let mut enforcer = self.enforcer.write().await;
        let result = enforcer.update_policy(old_rule, new_rule).await?;
        Ok(result)
    }

    pub async fn add_policies(&self, rules: Vec<Vec<String>>) -> AppResult<bool> {
        let mut enforcer = self.enforcer.write().await;
        let result = enforcer.add_policies(rules).await?;
        Ok(result)
    }

    pub async fn remove_policies(&self, rules: Vec<Vec<String>>) -> AppResult<bool> {
        let mut enforcer = self.enforcer.write().await;
        let result = enforcer.remove_policies(rules).await?;
        Ok(result)
    }

    pub async fn get_filtered_policy(
        &self,
        field_index: usize,
        field_values: Vec<String>,
    ) -> AppResult<Vec<Vec<String>>> {
        let enforcer = self.enforcer.read().await;
        let policies = enforcer.get_filtered_policy(field_index, field_values);
        Ok(policies)
    }

    // Database operations for casbin rules
    pub async fn get_all_rules_from_db(&self) -> AppResult<Vec<CasbinRule>> {
        let rows = sqlx::query_as!(
            CasbinRule,
            r#"
            SELECT 
                id,
                ptype,
                v0,
                v1,
                v2,
                v3,
                v4,
                v5
            FROM casbin_rule
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(rows)
    }

    pub async fn get_rules_by_filter(&self, filter: Filter) -> AppResult<Vec<CasbinRule>> {
        // Build dynamic query based on filter
        let mut query = String::from(
            r#"
            SELECT 
                id,
                ptype,
                v0,
                v1,
                v2,
                v3,
                v4,
                v5
            FROM casbin_rule
            WHERE 1=1
            "#,
        );
        let mut params: Vec<String> = Vec::new();

        if !filter.ptype.is_empty() {
            query.push_str(" AND ptype = $1");
            params.push(filter.ptype[0].clone());
        }
        if !filter.v0.is_empty() {
            let param_num = params.len() + 1;
            query.push_str(&format!(" AND v0 = ${}", param_num));
            params.push(filter.v0[0].clone());
        }
        if !filter.v1.is_empty() {
            let param_num = params.len() + 1;
            query.push_str(&format!(" AND v1 = ${}", param_num));
            params.push(filter.v1[0].clone());
        }
        if !filter.v2.is_empty() {
            let param_num = params.len() + 1;
            query.push_str(&format!(" AND v2 = ${}", param_num));
            params.push(filter.v2[0].clone());
        }
        if !filter.v3.is_empty() {
            let param_num = params.len() + 1;
            query.push_str(&format!(" AND v3 = ${}", param_num));
            params.push(filter.v3[0].clone());
        }
        if !filter.v4.is_empty() {
            let param_num = params.len() + 1;
            query.push_str(&format!(" AND v4 = ${}", param_num));
            params.push(filter.v4[0].clone());
        }
        if !filter.v5.is_empty() {
            let param_num = params.len() + 1;
            query.push_str(&format!(" AND v5 = ${}", param_num));
            params.push(filter.v5[0].clone());
        }

        let mut query_builder = sqlx::QueryBuilder::new(&query);
        for (i, param) in params.iter().enumerate() {
            if i == 0 {
                query_builder.bind(param);
            } else {
                query_builder.bind(param);
            }
        }
        
        let query = query_builder.build_query_as::<CasbinRule>();
        let rows = query.fetch_all(&self.pool).await.map_err(AppError::DatabaseError)?;
        Ok(rows)
    }

    pub async fn add_rule_to_db(
        &self,
        ptype: String,
        v0: Option<String>,
        v1: Option<String>,
        v2: Option<String>,
        v3: Option<String>,
        v4: Option<String>,
        v5: Option<String>,
    ) -> AppResult<CasbinRule> {
        let row = sqlx::query_as!(
            CasbinRule,
            r#"
            INSERT INTO casbin_rule (ptype, v0, v1, v2, v3, v4, v5)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, ptype, v0, v1, v2, v3, v4, v5
            "#,
            ptype,
            v0,
            v1,
            v2,
            v3,
            v4,
            v5
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(row)
    }

    pub async fn update_rule_in_db(
        &self,
        id: i32,
        ptype: Option<String>,
        v0: Option<String>,
        v1: Option<String>,
        v2: Option<String>,
        v3: Option<String>,
        v4: Option<String>,
        v5: Option<String>,
    ) -> AppResult<CasbinRule> {
        let row = sqlx::query_as!(
            CasbinRule,
            r#"
            UPDATE casbin_rule
            SET 
                ptype = COALESCE($2, ptype),
                v0 = COALESCE($3, v0),
                v1 = COALESCE($4, v1),
                v2 = COALESCE($5, v2),
                v3 = COALESCE($6, v3),
                v4 = COALESCE($7, v4),
                v5 = COALESCE($8, v5)
            WHERE id = $1
            RETURNING id, ptype, v0, v1, v2, v3, v4, v5
            "#,
            id,
            ptype,
            v0,
            v1,
            v2,
            v3,
            v4,
            v5
        )
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(row)
    }

    pub async fn delete_rule_from_db(&self, id: i32) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"
            DELETE FROM casbin_rule
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_casbin_service() {
        // This test requires a running PostgreSQL instance
        // For now, we'll just verify the structure compiles
        assert!(true);
    }
}