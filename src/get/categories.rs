pub mod category_public_id;
pub use self::category_public_id::ClubhouseGetCategoryCategoryPublicId;

pub struct ClubhouseGetCategory {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseGetCategory {
    pub fn category_public_id(
        self,
        category_public_id: u64,
    ) -> self::category_public_id::ClubhouseGetCategoryCategoryPublicId {
        self::category_public_id::ClubhouseGetCategoryCategoryPublicId {
            path: self.path.push(&category_public_id),
        }
    }

    /// See https://clubhouse.io/api/rest/v2/#List-Categories
    pub fn run(self) -> crate::Result<Vec<crate::types::Category>> {
        self.path
            .execute_as_json::<(), Vec<crate::types::Category>>(None)
    }
}
