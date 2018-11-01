pub mod category_public_id;
pub use self::category_public_id::ClubhousePutCategoryCategoryPublicId;

pub struct ClubhousePutCategory {
    pub(crate) path: burgundy::Path,
}

impl ClubhousePutCategory {
    pub fn category_public_id(
        self,
        category_public_id: u64,
    ) -> self::category_public_id::ClubhousePutCategoryCategoryPublicId {
        self::category_public_id::ClubhousePutCategoryCategoryPublicId {
            path: self.path.push(&category_public_id),
        }
    }
}
