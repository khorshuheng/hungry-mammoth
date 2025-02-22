use hungry_mammoth::{
  dto::user::*,
  repository::{
    error::RepositoryError,
    user::{UserPostgresRepository, UserRepository},
  },
};
use uuid::Uuid;

#[sqlx::test]
pub async fn test_user_crud(pool: sqlx::PgPool) {
  let user_repository = UserPostgresRepository::new(pool);
  let email = "user@domain.com".to_string();
  let new_user = user_repository
    .new_user(NewUserProfile {
      email: email.clone(),
      password_hash: "password_hash".to_string(),
    })
    .await
    .expect("new user creation failed");
  assert_eq!(new_user.email, email);
  let duplicate_email_error = user_repository
    .new_user(NewUserProfile {
      email: email.clone(),
      password_hash: "password_hash".to_string(),
    })
    .await
    .expect_err("should throw error when creating user with duplicated email");
  assert!(matches!(
    duplicate_email_error,
    RepositoryError::UniqueConstraintViolation(_)
  ));
  let user_uuid = new_user.uuid;
  let get_user = user_repository
    .get_user_by_uuid(user_uuid)
    .await
    .expect("failed to get user")
    .expect("user not found");
  assert_eq!(get_user.email, email);
  assert!(user_repository
    .get_user_by_uuid(Uuid::nil())
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
      uuid: user_uuid,
      email: email.clone()
    }]
  );
  let updated_email = "updated@domain.com".to_string();
  user_repository
    .update_user_by_uuid(
      user_uuid,
      UserProfileChange {
        email: Some(updated_email.clone()),
        password_hash: Some("password_hash".to_string()),
      },
    )
    .await
    .expect("failed to update user");
  user_repository
    .update_user_by_uuid(
      Uuid::nil(),
      UserProfileChange {
        email: Some(updated_email.clone()),
        password_hash: Some("password_hash".to_string()),
      },
    )
    .await
    .expect_err("should throw error when updating non existant user");
  let updated_user = user_repository
    .get_user_by_uuid(user_uuid)
    .await
    .expect("failed to get user")
    .expect("user not found");
  assert_eq!(updated_user.email, updated_email);
  user_repository
    .delete_user_by_uuid(user_uuid)
    .await
    .expect("failed to delete user");
  let users = user_repository
    .list_users()
    .await
    .expect("failed to list users");
  assert_eq!(users.len(), 0);
}
