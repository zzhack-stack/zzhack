use crate::services::provider_service::RootMetadata;
use web_sys::Storage;

pub struct CacheService {
    storage: Storage,
}

const ROOT_METADATA_STORAGE_KEY: &'static str = "root_metadata";

impl CacheService {
    pub fn new() -> CacheService {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        CacheService { storage }
    }

    pub fn set_root_metadata(&self, metadata: RootMetadata) {
        let serialize_metadata = serde_json::to_string(&metadata).unwrap();

        self.storage
            .set_item(ROOT_METADATA_STORAGE_KEY, serialize_metadata.as_str())
            .unwrap();
    }

    pub fn get_root_metadata(&self) -> RootMetadata {
        let serialize_metadata = self
            .storage
            .get_item(ROOT_METADATA_STORAGE_KEY)
            .unwrap()
            .unwrap();
        serde_json::from_str(&serialize_metadata).unwrap()
    }
}
