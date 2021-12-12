use crate::services::provider_service::PostMetadata;
use crate::CacheService;
use crate::RootMetadata;

pub struct PostService {
    root_metadata: RootMetadata,
}

impl PostService {
    pub fn new() -> PostService {
        let cache_service = CacheService::new();

        PostService {
            root_metadata: cache_service.get_root_metadata(),
        }
    }

    pub fn get_post_metadata(&self, category_name: &str, filename: &str) -> PostMetadata {
        let category = self.root_metadata.categories[category_name].clone();
        let target_metadata = category
            .iter()
            .find(|metadata| metadata.filename == filename)
            .unwrap();

        target_metadata.clone()
    }
}
