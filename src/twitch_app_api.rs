use anyhow::Result;
use serde::Deserialize;

const TWITCH_API_BASE_URL: &str = "https://addons-ecs.forgesvc.net/api/v2";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Addon {
    pub id: f64,
    pub name: String,
    pub authors: Vec<Author>,
    pub attachments: Vec<Attachment>,
    pub website_url: String,
    pub game_id: f64,
    pub summary: String,
    pub default_file_id: f64,
    pub download_count: f64,
    pub latest_files: Vec<File>,
    pub categories: Vec<PartialCategory>,
    pub status: u32,
    pub primary_category_id: u32,
    pub category_section: CategorySection,
    pub slug: String,
    pub game_version_latest_files: Vec<GameVersionFile>,
    pub is_featured: bool,
    pub popularity_score: f64,
    pub game_popularity_rank: f64,
    pub primary_language: String,
    pub game_slug: String,
    pub game_name: String,
    pub portal_name: String,
    pub date_modified: String,
    pub date_created: String,
    pub date_released: String,
    pub is_available: bool,
    #[serde(rename = "isExperiemental")]
    pub is_experimental: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameVersionFile {
    pub game_version: String,
    pub project_file_id: f64,
    pub project_file_name: String,
    pub file_type: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CategorySection {
    pub id: u32,
    pub game_id: u32,
    pub name: String,
    pub package_type: u32,
    pub path: String,
    pub initial_inclusion_pattern: String,
    pub extra_include_pattern: Option<String>,
    pub game_category_id: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PartialCategory {
    pub category_id: u32,
    pub name: String,
    pub url: String,
    pub avatar_url: String,
    pub parent_id: u32,
    pub root_id: u32,
    pub project_id: f64,
    pub avatar_id: u16,
    pub game_id: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub url: String,
    pub project_id: f64,
    pub id: f64,
    pub project_title_id: Option<f64>,
    pub project_title_title: Option<String>,
    pub user_id: f64,
    pub twitch_id: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: f64,
    pub project_id: f64,
    pub description: String,
    pub is_default: bool,
    pub thumbnail_url: String,
    pub title: String,
    pub url: String,
    pub status: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: f64,
    pub display_name: String,
    pub file_name: String,
    pub file_date: String,
    pub file_length: u32,
    pub release_type: u16,
    pub file_status: u16,
    pub download_url: String,
    pub is_alternate: bool,
    pub alternate_file_id: u32,
    pub dependencies: Vec<File>,
    pub is_available: bool,
    pub modules: Vec<Module>,
    pub package_fingerprint: f64,
    pub game_version: Vec<String>,
    pub sortable_game_version: Vec<SortableGameVersion>,
    pub install_metadata: Option<String>,
    pub changelog: Option<String>,
    pub has_install_script: bool,
    pub is_compatible_with_client: bool,
    pub category_section_package_type: u16,
    pub restrict_project_file_access: u16,
    pub project_status: u16,
    pub render_cache_id: f64,
    pub file_legacy_mapping_id: Option<f64>,
    pub project_id: f64,
    pub parent_project_file_id: Option<f64>,
    pub parent_file_legacy_mapping_id: Option<f64>,
    pub file_type_id: Option<f64>,
    pub expose_as_alternative: Box<Option<File>>,
    pub package_fingerprint_id: f64,
    pub game_version_date_released: String,
    pub game_version_mapping_id: f64,
    pub game_version_id: u32,
    pub game_id: u32,
    pub is_server_pack: bool,
    pub server_pack_file_id: Option<u32>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    #[serde(rename = "foldername")]
    pub folder_name: String,
    pub fingerprint: f64,
    #[serde(rename = "type")]
    pub module_type: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    pub game_version_padded: String,
    pub game_version: String,
    pub game_version_release_date: String,
    pub game_version_name: String,
}

impl Addon {
    pub async fn from_id(client: &reqwest::Client, id: u64) -> Result<Self> {
        let addon = client
            .get(&*format!("{}/addon/{}", TWITCH_API_BASE_URL, id))
            .send()
            .await?;
        let addon: Self = addon.json().await?;

        Ok(addon)
    }
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub id: f64,
    pub name: String,
    pub slug: String,
    pub avatar_url: String,
    pub date_modified: String,
    pub parent_game_category_id: f32,
    pub root: f32,
    pub game_id: f32,
}

impl Category {
    pub async fn from_id(client: &reqwest::Client, id: u64) -> Result<Self> {
        let category = client
            .get(&*format!("{}/category/{}", TWITCH_API_BASE_URL, id))
            .send()
            .await?;
        let category: Self = category.json().await?;

        Ok(category)
    }

    pub async fn section_from_id(client: reqwest::Client, id: u64) -> Result<Vec<Self>> {
        let section = client
            .get(&*format!("{}/category/section/{}", TWITCH_API_BASE_URL, id))
            .send()
            .await?;
        let section: Vec<Category> = section.json().await?;

        Ok(section)
    }
}

pub fn build_api_ready_client() -> Result<reqwest::Client> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    let client = reqwest::Client::builder()
        .user_agent("twitch_app_api_rs/0.1.3")
        .default_headers(headers)
        .build()?;
    Ok(client)
}
