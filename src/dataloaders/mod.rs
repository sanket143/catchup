use crate::schemas::contest::Contest;
use dataloader::BatchFn;
use dataloader::cached::Loader;
use futures::executor::block_on;
use futures::future::ready;
use sqlx::PgPool;
use std::collections::HashMap;
use std::thread;

struct ContestsByProblemTagGroupIdBatcher {
    db_pool: &'static PgPool,
}

impl ContestsByProblemTagGroupIdBatcher {
    pub fn new(db_pool: &'static PgPool) -> Self {
        Self { db_pool }
    }
}

impl BatchFn<usize, Vec<Contest>> for ContestsByProblemTagGroupIdBatcher {
    async fn load(&mut self, keys: &[usize]) -> HashMap<usize, Vec<Contest>> {
        // let user = ctx
        //     .user
        //     .as_ref()
        //     .ok_or(FieldError::new("User is not logged in", graphql_value!({})))?;

        // let rows = sqlx::query(
        //     r#"
        //         select c.id from problem_tag_group as ptg
        //         join contest as c
        //         on c.fk_problem_tag_group_id = ptg.id
        //         and c.is_deleted = false
        //         where ptg.id = $1
        //         and c.created_for = $2
        //     "#,
        // )
        // .bind(self.id)
        // .bind(user.username.clone())
        // .fetch_all(&*ctx.db_pool)
        // .await?;

        // let contest_ids = stream::iter(rows.into_iter().map(|row| {
        //     let id = row.try_get("id").unwrap_or_default();
        //     async move { Contest::by_id(ctx, &id).await }
        // }));

        // let contests: Vec<Contest> = contest_ids.buffer_unordered(10).try_collect().await?;

        // Ok(contests);

        println!("BatchFn load keys {:?}", keys);

        let ret = keys
            .iter()
            .map(|v| (v.clone(), vec![]))
            .collect::<HashMap<_, _>>();

        ready(ret).await
    }
}

pub struct Dataloaders {
    contestsByProblemTagGroupId: ContestsByProblemTagGroupIdBatcher,
}

type ContestsByProblemTagGroupIdLoader = Loader<i32, Contest, ContestsByProblemTagGroupIdBatcher>;

impl Dataloaders {
    pub fn new(db_pool: &'static PgPool) -> Self {
        Self {
            contestsByProblemTagGroupId: ContestsByProblemTagGroupIdBatcher::new(db_pool),
        }
    }
}
