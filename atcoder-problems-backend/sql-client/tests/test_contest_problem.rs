use sql_client::contest_problem::ContestProblemClient;
use sql_client::models::ContestProblem;
use sql_client::PgPool;

mod utils;

fn create_problem(id: i32) -> ContestProblem {
    ContestProblem {
        contest_id: format!("contest{}", id),
        problem_id: format!("problem{}", id),
        problem_index: format!("{}", id),
    }
}

#[sqlx::test]
async fn test_contest_problem(pool: PgPool) {
    utils::initialize(&pool).await;

    assert!(pool.load_contest_problem().await.unwrap().is_empty());

    pool.insert_contest_problem(&[create_problem(1), create_problem(2)])
        .await
        .unwrap();
    assert_eq!(
        pool.load_contest_problem().await.unwrap(),
        vec![create_problem(1), create_problem(2)]
    );
    pool.insert_contest_problem(&[create_problem(1)])
        .await
        .unwrap();
    assert_eq!(
        pool.load_contest_problem().await.unwrap(),
        vec![create_problem(1), create_problem(2)]
    );
}
