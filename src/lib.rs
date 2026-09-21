use gpui_kit::*;
use include_dir::{include_dir, Dir};
use std::borrow::Cow;
static ICONS: Dir = include_dir!("lucide_icons");
include!(concat!(env!("OUT_DIR"), "/lucide_icons.rs"));

pub struct LucideAssets;

impl AssetSource for LucideAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }
        let stripped_path = path.strip_prefix("lucide_icons/").unwrap_or(path);

        Ok(ICONS
            .get_file(stripped_path)
            .map(|file| Cow::Borrowed(file.contents())))
    }

    fn list(&self, _: &str) -> Result<Vec<SharedString>> {
        Ok(Vec::new())
    }
}
