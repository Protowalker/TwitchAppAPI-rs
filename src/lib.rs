mod twitch_app_api;

pub use crate::twitch_app_api::{
    Addon, Attachment, Author, Category, CategorySection, File, GameVersionFile, Module,
    SortableGameVersion,
};

#[cfg(test)]
mod tests {
    use super::twitch_app_api;
    use super::Addon;

    #[test]
    fn get_addon() {
        let client = twitch_app_api::build_api_ready_client().unwrap();
        tokio_test::block_on(Addon::from_id(&client, 238222)).unwrap();
    }
}
