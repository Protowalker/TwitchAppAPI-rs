mod twitch_app_api;

pub use crate::twitch_app_api::{
    Addon, Attachment, Author, Category, CategorySection, File, GameVersionFile, Module,
    SortableGameVersion,
};

#[cfg(test)]
mod tests {
    use super::Addon;

    #[test]
    fn get_addon() {
        tokio_test::block_on(Addon::from_id(238222)).unwrap();
    }
}
