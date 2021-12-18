use crate::services::github_service::GitHubProfile;
use crate::services::provider_service::RootMetadata;
use serde::de::DeserializeOwned;
use serde::Serialize;
use web_sys::Storage;

pub struct CacheService {
    storage: Storage,
}

const ROOT_METADATA_STORAGE_KEY: &'static str = "root_metadata";
const ACCESS_KEY_KEY: &'static str = "access_key";
const GITHUB_PROFILE_KEY: &'static str = "github_profile_key";

impl CacheService {
    pub fn new() -> CacheService {
        let storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();

        CacheService { storage }
    }

    fn set_value<T>(&self, key: &'static str, value: T)
    where
        T: Serialize,
    {
        let serilize_value = serde_json::to_string(&value).unwrap();

        self.storage.set_item(key, serilize_value.as_str()).unwrap();
    }

    fn get_value<T>(&self, key: &'static str) -> T
    where
        T: DeserializeOwned,
    {
        let serialize_metadata = self.storage.get_item(key).unwrap().unwrap();

        serde_json::from_str(&serialize_metadata).unwrap()
    }

    fn get_value_optionally<T>(&self, key: &'static str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        let geted_value = self.storage.get_item(key).unwrap();

        match geted_value {
            Some(value) => serde_json::from_str(&value).unwrap(),
            None => None,
        }
    }

    pub fn set_github_profile(&self, profile: &GitHubProfile) {
        self.set_value(GITHUB_PROFILE_KEY, profile);
    }

    pub fn get_github_profile(&self) -> Option<GitHubProfile> {
        self.get_value_optionally(GITHUB_PROFILE_KEY)
    }

    pub fn set_root_metadata(&self, metadata: RootMetadata) {
        self.set_value(ROOT_METADATA_STORAGE_KEY, metadata);
    }

    pub fn get_root_metadata(&self) -> RootMetadata {
        self.get_value(ROOT_METADATA_STORAGE_KEY)
    }

    pub fn set_github_access_key(&self, access_key: &str) {
        self.storage.set_item(ACCESS_KEY_KEY, access_key).unwrap()
    }

    pub fn get_github_access_key(&self) -> Option<String> {
        self.storage.get_item(ACCESS_KEY_KEY).unwrap()
    }
}
