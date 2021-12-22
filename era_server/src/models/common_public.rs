use crate::attr;

impl super::FullProfile {
    pub fn new_common_public(account_id: String) -> Self {
        let mut full = Self::new(String::from("common_public"), account_id);
        let attributes = &mut full.profile.stats.attributes;
        
        attr!(attributes, "banner_color", "");
        attr!(attributes, "banner_icon", "");
        attr!(attributes, "homebase_name", "Project Era");
        
        full
    }
}