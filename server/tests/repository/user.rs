use hungry_mammoth::{
  dto::user::{NewUserParameters, UpdateUserParameters, UserProfile},
  repository::user::{UserPostgresRepository, UserRepository},
};

#[sqlx::test]
pub async fn test_user_crud(pool: sqlx::PgPool) {
  let user_repository = UserPostgresRepository::new(pool);
  let email = "user@domain.com".to_string();
  let password = "password".to_string();
  let new_user = user_repository
    .new_user(NewUserParameters {
      email: email.clone(),
      password: password.clone(),
    })
    .await
    .expect("new user creation failed");
  assert_eq!(new_user.email, email);
  let user_id = new_user.id;
  let get_user = user_repository
    .get_user(user_id)
    .await
    .expect("failed to get user")
    .expect("user not found");
  assert_eq!(get_user.email, email);
  assert!(user_repository
    .get_user(0)
    .await
    .expect("failed to get user")
    .is_none());
  let list_user = user_repository
    .list_users()
    .await
    .expect("failed to list users");
  assert_eq!(
    list_user,
    vec![UserProfile {
      id: user_id,
      email: email.clone()
    }]
  );
  let updated_email = "updated@domain.com".to_string();
  user_repository
    .update_user(
      user_id,
      UpdateUserParameters {
        email: updated_email.clone(),
        password: password.clone(),
      },
    )
    .await
    .expect("failed to update user");
  user_repository
    .update_user(
      0,
      UpdateUserParameters {
        email: updated_email.clone(),
        password: password.clone(),
      },
    )
    .await
    .expect_err("should throw error when updating non existant user");
  let updated_user = user_repository
    .get_user(user_id)
    .await
    .expect("failed to get user")
    .expect("user not found");
  assert_eq!(updated_user.email, updated_email);
  user_repository
    .delete_user(user_id)
    .await
    .expect("failed to delete user");
  let users = user_repository
    .list_users()
    .await
    .expect("failed to list users");
  assert_eq!(users.len(), 0);
}
