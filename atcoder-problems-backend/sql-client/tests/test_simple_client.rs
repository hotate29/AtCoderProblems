use sql_client::models::{Contest, Problem};
use sql_client::simple_client::SimpleClient;
use sql_client::PgPool;

mod utils;

#[sqlx::test]
async fn test_insert_contests(pool: PgPool) {
    utils::initialize(&pool).await;

    assert!(pool.load_contests().await.unwrap().is_empty());
    pool.insert_contests(&[Contest {
        id: "contest1".to_string(),
        start_epoch_second: 0,
        duration_second: 0,
        title: "".to_string(),
        rate_change: "".to_string(),
    }])
    .await
    .unwrap();

    let contests = pool.load_contests().await.unwrap();
    assert_eq!(contests[0].id, "contest1");

    pool.insert_contests(&[Contest {
        id: "contest1".to_string(),
        start_epoch_second: 0,
        duration_second: 0,
        title: "".to_string(),
        rate_change: "".to_string(),
    }])
    .await
    .unwrap();
}

#[sqlx::test]
async fn test_insert_problems(pool: PgPool) {
    utils::initialize(&pool).await;

    assert!(pool.load_problems().await.unwrap().is_empty());
    pool.insert_problems(&[Problem {
        id: "problem1".to_string(),
        contest_id: "".to_string(),
        problem_index: "".to_string(),
        name: "".to_string(),
        title: "".to_string(),
    }])
    .await
    .unwrap();

    let problems = pool.load_problems().await.unwrap();
    assert_eq!(problems[0].id, "problem1");

    pool.insert_problems(&[Problem {
        id: "problem1".to_string(),
        contest_id: "".to_string(),
        problem_index: "".to_string(),
        name: "".to_string(),
        title: "".to_string(),
    }])
    .await
    .unwrap();
}
