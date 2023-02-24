use sql_client::{
    internal::progress_reset_manager::{
        ProgressResetItem, ProgressResetList, ProgressResetManager,
    },
    PgPool,
};

mod utils;

#[sqlx::test]
async fn test_progress_reset_manager(pool: PgPool) {
    let internal_user_id = "user_id";
    let atcoder_user_id = "atcoder_id";
    let problem_id = "problem_id";
    let reset_epoch_second = 42;

    utils::initialize(&pool).await;
    utils::setup_internal_user(&pool, internal_user_id, atcoder_user_id).await;

    let list = pool
        .get_progress_reset_list(internal_user_id)
        .await
        .unwrap();
    assert!(
        list.items.is_empty(),
        "`get_progress_reset_list` here should return an empty list, but got not empty."
    );

    pool.add_item(internal_user_id, problem_id, reset_epoch_second)
        .await
        .unwrap();
    let list = pool
        .get_progress_reset_list(internal_user_id)
        .await
        .unwrap();
    assert_eq!(
        list,
        ProgressResetList {
            items: vec![ProgressResetItem {
                problem_id: problem_id.to_string(),
                reset_epoch_second,
            }],
        },
        "The item that has been added to the list is not found."
    );

    let updated_reset_epoch_second = 334;
    pool.add_item(internal_user_id, problem_id, updated_reset_epoch_second)
        .await
        .unwrap();
    let list = pool
        .get_progress_reset_list(internal_user_id)
        .await
        .unwrap();
    assert_eq!(
        list,
        ProgressResetList {
            items: vec![ProgressResetItem {
                problem_id: problem_id.to_string(),
                reset_epoch_second: updated_reset_epoch_second,
            }],
        },
        "`reset_epoch_second` should be updated, but not."
    );

    pool.remove_item(internal_user_id, problem_id)
        .await
        .unwrap();
    let list = pool
        .get_progress_reset_list(internal_user_id)
        .await
        .unwrap();
    assert!(
        list.items.is_empty(),
        "The list should not have any items, but still has."
    );
}
