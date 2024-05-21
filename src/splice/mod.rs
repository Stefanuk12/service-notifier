use reqwest::Client;

mod json;
use crate::{circular_buffer::CircularBuffer, pack::*};
pub use json::*;

const JSON_STRING: &str = r#"{"operationName":"PacksSearch","variables":{"limit":5,"sort":"recency","tags":[],"tags_exclude":[],"order":"DESC","legacy":true},"query":"query PacksSearch(\n $asset_status_slug: AssetStatusSlug\n $page: Int\n $order: SortOrder\n $limit: Int = 60\n $sort: AssetSortType = relevance\n $random_seed: String\n $parent_asset_uuid: GUID\n $parent_asset_type: AssetTypeSlug\n $query: String\n $tags: [ID!]\n $tags_exclude: [ID!]\n $attributes: [AssetAttributeSlug!]\n $liked: Boolean\n $filepath: String\n $asset_category_slug: AssetCategorySlug\n $ac_uuid: String\n $licensed: Boolean\n $provider: GUID\n $legacy: Boolean\n) {\n assetsSearch(\n filter: {\n legacy: $legacy\n asset_type_slug: pack\n asset_status_slug: $asset_status_slug\n asset_category_slug: $asset_category_slug\n query: $query\n tag_ids: $tags\n tag_ids_exclude: $tags_exclude\n attributes: $attributes\n liked: $liked\n filepath: $filepath\n ac_uuid: $ac_uuid\n licensed: $licensed\n provider: $provider\n }\n children: { parent_asset_uuid: $parent_asset_uuid }\n pagination: { page: $page, limit: $limit }\n sort: { sort: $sort, order: $order, random_seed: $random_seed }\n legacy: { parent_asset_type: $parent_asset_type, use: $legacy }\n ) {\n ...assetDetails\n }\n}\nfragment assetDetails on AssetPage {\n ...assetPageItems\n}\nfragment assetPageItems on AssetPage {\n items {\n ... on PackAsset {\n uuid\n name\n provider {\n name\n }\n permalink_slug\n permalink_base_url\n }\n }\n}\n"}"#;

pub struct SpliceListener {
    client: Client,
    recent_uuids: CircularBuffer<String, 5>,
}
impl SpliceListener {
    /// Create a new [SpliceListener].
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            recent_uuids: CircularBuffer::new(),
        }
    }

    /// Get the most recent packs from Splice.
    async fn get_recent_packs(&self) -> Result<Vec<Item>, reqwest::Error> {
        let response = self
            .client
            .post("https://surfaces-graphql.splice.com/graphql")
            .body(JSON_STRING)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .json::<PacksSearchResult>()
            .await?;

        Ok(response.data.assets_search.items)
    }
}
impl PackListener for SpliceListener {
    async fn poll(&mut self) -> Result<impl Iterator<Item = Pack>, reqwest::Error> {
        let new_packs = self
            .get_recent_packs()
            .await?
            .into_iter()
            .filter_map(|pack| {
                if self.recent_uuids.contains(&pack.uuid) {
                    return None;
                }

                self.recent_uuids.push(pack.uuid.clone());

                Some(Pack {
                    provider: pack.provider.name.clone(),
                    pack: pack.name,
                    url: format!(
                        "https://splice.com/sounds/packs/{}/{}/samples",
                        pack.permalink_base_url, pack.permalink_slug
                    ),
                })
            });

        Ok(new_packs)
    }
}
