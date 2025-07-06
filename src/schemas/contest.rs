use juniper::{FieldResult, GraphQLInputObject, graphql_object};

use crate::context::Context;

use super::{
    contest_problem_map::ContestProblemMap, problem_tag_group::ProblemTagGroup, user::User,
};

#[derive(Debug)]
pub struct Contest {
    pub id: i64,
    pub name: String,
    pub duration: i64,
    pub level: i64,
    pub created_on: i64,
    pub started_on: i64,
    pub created_for: String,
    pub fk_problem_tag_group_id: i64,
    pub is_evaluated: bool,
}

#[graphql_object(Context = Context)]
impl Contest {
    fn id(&self) -> i32 {
        self.id as i32
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn duration(&self) -> i32 {
        self.duration as i32
    }

    fn created_on(&self) -> i32 {
        self.created_on as i32
    }

    fn started_on(&self) -> i32 {
        self.started_on as i32
    }

    fn created_for(&self) -> &String {
        &self.created_for
    }

    fn is_evaluated(&self) -> &bool {
        &self.is_evaluated
    }

    async fn problem_tag_group(&self, ctx: &Context) -> FieldResult<ProblemTagGroup> {
        Ok(ProblemTagGroup::by_id(ctx, &self.fk_problem_tag_group_id)
            .await
            .expect("Failed to get problem tag group for a contest"))
    }

    async fn problems(&self, ctx: &Context) -> Vec<ContestProblemMap> {
        ContestProblemMap::by_contest_id(ctx, &self.id)
            .await
            .expect("Unable to fetch contest problem map for a Contest")
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
    pub async fn by_id(ctx: &Context, contest_id: &i64) -> sqlx::Result<Self> {
        let mut tx = ctx.db_pool.begin().await?;

        sqlx::query_as!(
            Self,
            r#"
                select
                    c.id, c.name, c.duration, c.level, c.created_on,
                    c.started_on, c.created_for, c.fk_problem_tag_group_id,
                    c.is_evaluated
                from contest as c
                where c.id = ?
            "#,
            contest_id
        )
        .fetch_one(&mut *tx)
        .await
    }

    pub async fn create<'e, E>(
        tx: E,
        input: &CreateContestInput,
        duration: &i64,
        user: &User,
        problem_tag: &ProblemTagGroup,
    ) -> sqlx::Result<Self>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query_as!(
            Self,
            r#"
                insert into contest (name, duration, level, created_for, fk_problem_tag_group_id)
                values (?, ?, ?, ?, ?) returning id as "id!", name, duration, level,
                created_on, started_on, created_for, fk_problem_tag_group_id,
                is_evaluated;
            "#,
            input.name,
            duration,
            user.level,
            user.username,
            problem_tag.id
        )
        .fetch_one(tx)
        .await
    }

    pub async fn add_problem_by_uid<'e, E>(&self, tx: E, problem_uid: &str) -> sqlx::Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            r#"
                insert into contest_problem_map(fk_contest_id, fk_problem_id)
                select ?, p.id
                from problem as p
                where p.uid = ?;
            "#,
            self.id,
            problem_uid,
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn add_random_problem<'e, E>(
        &self,
        tx: E,
        problem_rating: &i64,
        problem_tag_group: &ProblemTagGroup,
    ) -> sqlx::Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            r#"
            insert into contest_problem_map(fk_contest_id, fk_problem_id)
            select ?, p.id
            from problem_tag_group as ptg
            join problem_tag as pt
            on pt.fk_problem_tag_group_id = ptg.id
            join problem_tag_map as ptm
            on ptm.fk_problem_tag_id = pt.id
            join problem as p
            on p.id = ptm.fk_problem_id
            and p.rating = ?
            left join contest_problem_map as cpm
            on cpm.fk_problem_id = p.id
            and cpm.fk_contest_id = ?
            and cpm.is_deleted = false
            where ptg.id = ?
            and cpm.id is null
            order by random() limit 1;
        "#,
            self.id,
            problem_rating,
            self.id,
            problem_tag_group.id,
        )
        .execute(tx)
        .await?;

        Ok(())
    }

    pub async fn mark_as_evaluate<'e, E>(&self, tx: E) -> sqlx::Result<()>
    where
        E: sqlx::Executor<'e, Database = sqlx::Sqlite>,
    {
        sqlx::query!(
            r#"
                update contest as c
                set is_evaluated = true
                where c.id = ?;
            "#,
            self.id,
        )
        .execute(tx)
        .await?;

        Ok(())
    }
}
