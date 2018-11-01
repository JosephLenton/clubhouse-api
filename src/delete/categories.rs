pub mod category_public_id;
pub use self::category_public_id::ClubhouseDeleteCategoryCategoryPublicId;

pub struct ClubhouseDeleteCategory {
    pub(crate) path: burgundy::Path,
}

impl ClubhouseDeleteCategory {
    pub fn category_public_id(
        self,
        category_public_id: u64,
    ) -> self::category_public_id::ClubhouseDeleteCategoryCategoryPublicId {
        self::category_public_id::ClubhouseDeleteCategoryCategoryPublicId {
            path: self.path.push(&category_public_id),
        }
    }
}
