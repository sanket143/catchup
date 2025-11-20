use juniper::{FieldError, FieldResult, GraphQLInputObject, graphql_object, graphql_value};
use sqlx::prelude::FromRow;

use crate::context::Context;

use super::{
    contest_problem_map::ContestProblemMap, problem_tag_group::ProblemTagGroup, user::User,
};

#[derive(Debug, FromRow, Clone)]
pub struct Contest {
    pub id: i32,
    pub name: String,
    pub duration: i32,
    pub level: i32,
    pub created_on: i32,
    pub started_on: i32,
    pub created_for: String,
    pub fk_problem_tag_group_id: i32,
    pub is_evaluated: bool,
}

#[graphql_object(Context = Context)]
impl Contest {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn duration(&self) -> i32 {
        self.duration
    }

    fn created_on(&self) -> i32 {
        self.created_on
    }

    fn started_on(&self) -> i32 {
        self.started_on
    }

    fn created_for(&self) -> &String {
        &self.created_for
    }

    fn is_evaluated(&self) -> &bool {
        &self.is_evaluated
    }

    async fn problem_tag_group(&self, ctx: &Context) -> FieldResult<ProblemTagGroup> {
        ProblemTagGroup::by_id(ctx, &self.fk_problem_tag_group_id)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Failed to get problem tag group for a contest: {:?}", e),
                    graphql_value!({}),
                )
            })
    }

    async fn problems(&self, ctx: &Context) -> FieldResult<Vec<ContestProblemMap>> {
        ContestProblemMap::by_contest_id(ctx, &self.id)
            .await
            .map_err(|e| {
                FieldError::new(
                    format!("Unable to fetch contest problem map for a Contest: {:?}", e),
                    graphql_value!({}),
                )
            })
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Contest creation input")]
pub struct CreateContestInput {
    pub name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "Contest evaluation input")]
pub struct EvaluateContestInput {
    pub contest_id: i32,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "End contest input (not in use at the moment)")]
pub struct EndContestInput {
    pub contest_id: i32,
}

impl Contest {
    pub async fn by_id(ctx: &Context, contest_id: &i32) -> sqlx::Result<Self> {
        let mut tx = ctx.db_pool.begin().await?;

        sqlx::query_as::<_, Self>(
            r#"
                select
                    c.id, c.name, c.duration, c.level, c.created_on,
                    c.started_on, c.created_for, c.fk_problem_tag_group_id,
                    c.is_evaluated
                from public.contest as c
                where c.id = $1
            "#,
        )
        .bind(*contest_id)
        .fetch_one(&mut *tx)
        .await
    }

    pub async fn create<'e, E>(
        tx: E,
        input: &CreateContestInput,
        duration: &i32,
        user: &User,
        problem_tag: &ProblemTagGroup,
    ) -> sqlx::Result<Self>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query_as::<_, Self>(
            r#"
                insert into contest (name, duration, level, created_for, fk_problem_tag_group_id)
                values ($1, $2, $3, $4, $5) returning id, name, duration, level,
                created_on, started_on, created_for, fk_problem_tag_group_id,
                is_evaluated
            "#,
        )
        .bind(input.name.clone())
        .bind(*duration)
        .bind(user.level)
        .bind(user.username.clone())
        .bind(problem_tag.id)
        .fetch_one(tx)
        .await
    }

    pub async fn add_problem_by_uid<'e, E>(&self, tx: E, problem_uid: &str) -> sqlx::Result<()>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query(
            r#"
                insert into contest_problem_map(fk_contest_id, fk_problem_id)
                select $1, p.id
                from problem as p
                where p.uid = $2;
            "#,
        )
        .bind(self.id)
        .bind(problem_uid)
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn add_random_problem<'e, E>(
        &self,
        tx: E,
        problem_rating: &i32,
        problem_tag_group: &ProblemTagGroup,
    ) -> sqlx::Result<()>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query(
            r#"
            insert into contest_problem_map(fk_contest_id, fk_problem_id)
            select $1, p.id
            from problem_tag_group as ptg
            join problem_tag as pt
            on pt.fk_problem_tag_group_id = ptg.id
            join problem_tag_map as ptm
            on ptm.fk_problem_tag_id = pt.id
            join problem as p
            on p.id = ptm.fk_problem_id
            and p.rating = $2
            left join contest_problem_map as cpm
            on cpm.fk_problem_id = p.id
            and cpm.fk_contest_id = $3
            and cpm.is_deleted = false
            where ptg.id = $4
            and cpm.id is null
            order by random() limit 1;
        "#,
        )
        .bind(self.id)
        .bind(*problem_rating)
        .bind(self.id)
        .bind(problem_tag_group.id)
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn mark_as_evaluate<'e, E>(&self, tx: E) -> sqlx::Result<()>
    where
        E: sqlx::PgExecutor<'e>,
    {
        sqlx::query(
            r#"
                update contest as c
                set is_evaluated = true
                where c.id = $1;
            "#,
        )
        .bind(self.id)
        .execute(tx)
        .await?;

        Ok(())
    }
}
