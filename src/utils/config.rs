// Configuration utilities
// Provides functions for accessing build-time configuration values

/// Get the base URL from build environment
/// This reads from the TRUNK_PUBLIC_URL environment variable set by our build script
/// which parses the public_url from Trunk.toml at compile time
/// Falls back to "/zzhack" if not set
pub fn get_base_url() -> &'static str {
    // Build script sets TRUNK_PUBLIC_URL by reading Trunk.toml
    // We use env! to read it at compile time (guaranteed to be set by build.rs)
    env!("TRUNK_PUBLIC_URL")
}

/// Build a full URL path by combining base URL with a relative path
/// 
/// # Arguments
/// * `path` - The relative path to append to the base URL
/// 
/// # Returns
/// A String containing the full URL path
/// 
/// # Examples
/// ```
/// let url = build_url("/data/file.txt");
/// // Returns "/zzhack/data/file.txt" (assuming default base URL)
/// ```
pub fn build_url(path: &str) -> String {
    let base_url = get_base_url();
    
    if path.starts_with('/') {
        // Path starts with /, append to base URL
        format!("{}{}", base_url, path)
    } else {
        // Path doesn't start with /, add separator
        format!("{}/{}", base_url, path)
    }
}

/// Build a data URL path for accessing files in the data directory
/// 
/// # Arguments
/// * `file_path` - The relative path within the data directory
/// 
/// # Returns
/// A String containing the full data URL path
/// 
/// # Examples
/// ```
/// let url = build_data_url("readme.md");
/// // Returns "/zzhack/data/readme.md" (assuming default base URL)
/// ```
pub fn build_data_url(file_path: &str) -> String {
    build_url(&format!("/data/{}", file_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_base_url() {
        // Should return a non-empty string
        let base_url = get_base_url();
        assert!(!base_url.is_empty());
    }

    #[test]
    fn test_build_url_with_leading_slash() {
        let url = build_url("/test/path");
        assert!(url.starts_with(get_base_url()));
        assert!(url.ends_with("/test/path"));
    }

    #[test]
    fn test_build_url_without_leading_slash() {
        let url = build_url("test/path");
        assert!(url.starts_with(get_base_url()));
        assert!(url.ends_with("/test/path"));
    }

    #[test]
    fn test_build_data_url() {
        let url = build_data_url("file.txt");
        assert!(url.starts_with(get_base_url()));
        assert!(url.ends_with("/data/file.txt"));
    }
}