mod repo_public_id;
pub use self::repo_public_id::ClubhouseGetRepositoryRepoPublicId;

pub struct ClubhouseGetRepository {
  pub(crate) path: burgundy::Path,
}

impl ClubhouseGetRepository {
  pub fn repo_public_id(self, repo_public_id : String) -> self::repo_public_id::ClubhouseGetRepositoryRepoPublicId {
    self::repo_public_id::ClubhouseGetRepositoryRepoPublicId {
      path: self.path.push(&repo_public_id),
    }
  }

  /// See https://clubhouse.io/api/rest/v2/#List-Repositories
  pub fn run(self) -> burgundy::Result<Vec<crate::types::Repository>> {
    self.path
        .execute_as_json::<Vec<crate::types::Repository>>()
  }
}
