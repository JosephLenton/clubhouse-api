
const URL_FORMAT_GENERIC_ITEM : &'static str = "https://app.clubhouse.io/{}/{}/{}/";

pub fn generate_url_story( org : &str, code : &str ) -> String {
  generate_generic_url( org, code, "story" )
}

pub fn generate_url_epic( org : &str, code : &str ) -> String {
  generate_generic_url( org, code, "epic" )
}

pub fn generate_url_generic_item(org: &str, code: &str, item_type: &str) -> String {
  format!( URL_FORMAT_GENERIC_ITEM, org, item_type, code )
}
