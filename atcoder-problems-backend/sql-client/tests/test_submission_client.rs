use sql_client::models::Submission;
use sql_client::submission_client::{SubmissionClient, SubmissionRequest};
use sql_client::PgPool;

mod utils;

#[sqlx::test]
async fn test_submission_client(pool: PgPool) {
    utils::initialize(&pool).await;

    sqlx::query(
        r"
        INSERT INTO submissions
            (id, epoch_second, problem_id, contest_id, user_id, language, point, length, result)
        VALUES
            (1, 100, 'problem1', 'contest1', 'user1', 'language1', 1.0, 1, 'AC'),
            (2, 200, 'problem1', 'contest1', 'user2', 'language1', 1.0, 1, 'AC'),
            (3, 300, 'problem1', 'contest1', 'user1', 'language1', 1.0, 1, 'WA'),
            (4, 400, 'problem1', 'contest1', 'user1', 'language1', 1.0, 1, 'AC'),
            (5, 1, 'problem2', 'contest1', 'userx', 'language1', 1.0, 1, '23/42 TLE'),
            (6, 2, 'problem2', 'contest1', 'userx', 'language1', 1.0, 1, '23/42 TLE');
    ",
    )
    .execute(&pool)
    .await
    .unwrap();

    let request = SubmissionRequest::UserAll { user_id: "usEr1" };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 3);

    let request = SubmissionRequest::UserAll { user_id: "user2" };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 1);

    let request = SubmissionRequest::UserAll { user_id: "user3" };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert!(submissions.is_empty());

    let request = SubmissionRequest::RecentAccepted { count: 0 };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert!(submissions.is_empty());

    let request = SubmissionRequest::RecentAccepted { count: 1 };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 1);

    let request = SubmissionRequest::RecentAccepted { count: 2 };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 2);

    let request = SubmissionRequest::RecentAccepted { count: 100 };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 3);

    let request = SubmissionRequest::FromTime {
        from_second: 100,
        count: 10,
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 4);

    let request = SubmissionRequest::FromTime {
        from_second: 200,
        count: 10,
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 3);

    let request = SubmissionRequest::FromTime {
        from_second: 100,
        count: 1,
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 1);

    let request = SubmissionRequest::FromUserAndTime {
        user_id: "usEr1",
        from_second: 300,
        count: 1000,
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 2);
    assert_eq!(submissions[0].result, "WA");
    assert_eq!(submissions[1].result, "AC");

    let request = SubmissionRequest::FromUserAndTime {
        user_id: "usEr1",
        from_second: 300,
        count: 1,
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 1);

    let request = SubmissionRequest::FromUserAndTime {
        user_id: "user3",
        from_second: 300,
        count: 1000,
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert!(submissions.is_empty());

    let request = SubmissionRequest::UsersAccepted {
        user_ids: &["user1", "user2"],
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 3);

    let request = SubmissionRequest::UsersAccepted {
        user_ids: &["user1"],
    };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 2);

    let submissions = pool
        .get_submissions(SubmissionRequest::AllAccepted)
        .await
        .unwrap();
    assert_eq!(submissions.len(), 3);

    assert_eq!(pool.count_stored_submissions(&[1]).await.unwrap(), 1);
    assert_eq!(pool.count_stored_submissions(&[9]).await.unwrap(), 0);

    let request = SubmissionRequest::InvalidResult { from_second: 1 };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 2);

    let request = SubmissionRequest::InvalidResult { from_second: 2 };
    let submissions = pool.get_submissions(request).await.unwrap();
    assert_eq!(submissions.len(), 1);
}

#[sqlx::test]
async fn test_update_submissions(pool: PgPool) {
    utils::initialize(&pool).await;

    pool.update_submissions(&[Submission {
        id: 0,
        user_id: "old_user_name".to_owned(),
        result: "WJ".to_owned(),
        point: 0.0,
        execution_time: None,
        ..Default::default()
    }])
    .await
    .unwrap();

    let submissions = pool
        .get_submissions(SubmissionRequest::UserAll {
            user_id: "old_user_name",
        })
        .await
        .unwrap();
    assert_eq!(submissions.len(), 1);
    assert_eq!(submissions[0].user_id, "old_user_name");
    assert_eq!(submissions[0].result, "WJ");
    assert_eq!(submissions[0].point, 0.0);
    assert_eq!(submissions[0].execution_time, None);

    let submissions = pool
        .get_submissions(SubmissionRequest::UserAll {
            user_id: "new_user_name",
        })
        .await
        .unwrap();
    assert!(submissions.is_empty());

    pool.update_submissions(&[Submission {
        id: 0,
        user_id: "new_user_name".to_owned(),
        result: "AC".to_owned(),
        point: 100.0,
        execution_time: Some(1),
        ..Default::default()
    }])
    .await
    .unwrap();

    let submissions = pool
        .get_submissions(SubmissionRequest::UserAll {
            user_id: "old_user_name",
        })
        .await
        .unwrap();
    assert!(submissions.is_empty());

    let submissions = pool
        .get_submissions(SubmissionRequest::UserAll {
            user_id: "new_user_name",
        })
        .await
        .unwrap();
    assert_eq!(submissions.len(), 1);
    assert_eq!(submissions[0].user_id, "new_user_name");
    assert_eq!(submissions[0].result, "AC");
    assert_eq!(submissions[0].point, 100.0);
    assert_eq!(submissions[0].execution_time, Some(1));
}
