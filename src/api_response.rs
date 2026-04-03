// Copyright (c) 2024 구FS, all rights reserved. Subject to the MIT licence in `licence.md`.
use std::str::FromStr;


// ====== v2 API: Gallery Detail (GET /api/v2/galleries/{id}) ======

/// # Summary
/// Gallery detail response from "nhentai.net/api/v2/galleries/{gallery_id}".
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GalleryDetailResponse
{
    pub id: u32,
    pub media_id: String,
    pub title: GalleryTitle,
    pub cover: CoverInfo,
    pub thumbnail: CoverInfo,
    pub scanlator: Option<String>,
    pub upload_date: u64, // unix timestamp
    pub tags: Vec<TagResponse>,
    pub num_pages: u16,
    pub num_favorites: u32,
    pub pages: Vec<PageInfo>,
}


/// # Summary
/// Gallery title in multiple languages (v2 API).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GalleryTitle
{
    pub english: String,
    pub japanese: Option<String>,
    pub pretty: String,
}


/// # Summary
/// Cover/thumbnail image info (v2 API).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CoverInfo
{
    pub path: String, // e.g. "1234567/1.jpg"
    pub width: u32,
    pub height: u32,
}


/// # Summary
/// Page/image info for a gallery (v2 API).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PageInfo
{
    pub number: u16, // 1-based page number
    pub path: String, // e.g. "1234567/1.jpg"
    pub width: u32,
    pub height: u32,
    pub thumbnail: String,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
}


/// # Summary
/// Tag response from v2 API (same shape as old Tag struct).
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TagResponse
{
    pub id: u32,
    pub r#type: String,
    pub name: String,
    pub slug: String,
    pub url: String,
    pub count: u32,
}


// ====== v2 API: Search (GET /api/v2/search) ======

/// # Summary
/// Search response from "nhentai.net/api/v2/search?query={tags}&page={n}".
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct SearchResponse
{
    pub result: Vec<GalleryListItem>,
    pub num_pages: u32,
    pub per_page: u16,
    pub total: Option<u32>,
}


/// # Summary
/// Lightweight gallery item from search results (v2 API).
/// Contains only tag_ids, not full tag objects.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct GalleryListItem
{
    pub id: u32,
    pub media_id: String,
    pub english_title: String,
    pub japanese_title: Option<String>,
    pub thumbnail: String,
    pub thumbnail_width: u32,
    pub thumbnail_height: u32,
    pub num_pages: u16,
    pub tag_ids: Vec<u32>,
    pub blacklisted: bool,
}


// ====== v2 API: CDN Config (GET /api/v2/cdn) ======

/// # Summary
/// CDN configuration response from "nhentai.net/api/v2/cdn".
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[allow(dead_code)]
pub struct CdnConfigResponse
{
    pub image_servers: Vec<String>,
    pub thumb_servers: Vec<String>,
}


// ====== ImageType enum (still needed for DB storage and URL construction) ======

#[derive(Clone, Eq, PartialEq)]
pub enum ImageType
{
    Gif,
    Jpg,
    Png,
    Webp,
}

impl<'de> serde::Deserialize<'de> for ImageType
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> // str -> ImageType
    where
        D: serde::Deserializer<'de>,
    {
        let s_de: String = String::deserialize(deserializer)?;
        match Self::from_str(s_de.as_str())
        {
            Ok(o) => return Ok(o),
            _ => return Err(serde::de::Error::custom(format!("Invalid image type: \"{s_de}\""))),
        };
    }
}

impl serde::Serialize for ImageType
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> // ImageType -> str
    where
        S: serde::Serializer,
    {
        let s: String = format!("{:?}", self);
        return serializer.serialize_str(s.as_str());
    }
}

impl std::fmt::Debug for ImageType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result // ImageType -> str
    {
        return write!(f, "{}",
            match self
            {
                Self::Gif => "g", // only short form in program context (database)
                Self::Jpg => "j",
                Self::Png => "p",
                Self::Webp => "w",
            }
        );
    }
}

impl std::fmt::Display for ImageType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result // ImageType -> str
    {
        return write!(f, "{}",
            match self
            {
                Self::Gif => "gif", // long form for output
                Self::Jpg => "jpg",
                Self::Png => "png",
                Self::Webp => "webp",
            }
        );
    }
}

impl std::str::FromStr for ImageType
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> // str -> ImageType
    {
        let image_type: ImageType = match s.to_lowercase().trim()
        {
            "g" | "gif" => Self::Gif,
            "j" | "jpg" => Self::Jpg,
            "p" | "png" => Self::Png,
            "w" | "webp" => Self::Webp,
            _ => return Err(format!("Invalid image type: \"{s}\"")),
        };
        return Ok(image_type);
    }
}


/// # Summary
/// Extract ImageType from a file path like "1234567/1.jpg".
pub fn image_type_from_path(path: &str) -> Result<ImageType, String>
{
    let ext = path.rsplit('.').next().ok_or_else(|| format!("No extension found in path: \"{path}\""))?;
    ImageType::from_str(ext)
}


// ====== Tag struct (for DB, same shape as old version) ======

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Tag
{
    pub id: u32,
    pub name: String,
    pub r#type: String, // type is a reserved keyword, r#type resolves to type
    pub url: String,
}


// ====== Database write for GalleryDetailResponse ======

impl GalleryDetailResponse
{
    /// # Summary
    /// Write gallery detail response to database. Either creates new entries or updates existing ones with same primary key.
    ///
    /// # Arguments
    /// - `db`: SQLite database
    ///
    /// # Returns
    /// - number of rows affected or sqlx::Error
    pub async fn write_to_db(&self, db: &sqlx::sqlite::SqlitePool) -> Result<u64, sqlx::Error>
    {
        const HENTAI_QUERY_STRING: &str = "INSERT OR REPLACE INTO Hentai (id, cover_type, media_id, num_favorites, num_pages, page_types, scanlator, title_english, title_japanese, title_pretty, upload_date) ";
        const HENTAI_TAG_QUERY1_STRING: &str = "DELETE FROM Hentai_Tag WHERE hentai_id = ";
        const HENTAI_TAG_QUERY2_STRING: &str = "INSERT INTO Hentai_Tag (hentai_id, tag_id) ";
        const TAG_QUERY_STRING: &str = "INSERT OR REPLACE INTO Tag (id, name, type, url) ";
        let mut db_tx: sqlx::Transaction<'_, sqlx::Sqlite>;
        let mut rows_affected: u64 = 0;


        db_tx = db.begin_with("PRAGMA foreign_keys = OFF; BEGIN TRANSACTION;").await?;

        // Convert upload_date from unix timestamp to chrono::DateTime
        let upload_date = chrono::DateTime::from_timestamp(self.upload_date as i64, 0)
            .ok_or_else(|| sqlx::Error::Decode(format!("Invalid unix timestamp: {}", self.upload_date).into()))?;

        // Build page_types string from pages[].path
        let page_types: String = self.pages.iter()
            .map(|page|
            {
                match image_type_from_path(&page.path)
                {
                    Ok(t) => format!("{t:?}"),
                    Err(e) =>
                    {
                        log::warn!("Could not determine image type from path \"{}\": {e}", page.path);
                        "j".to_owned() // fallback to jpg
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("");

        // Build cover_type from cover.path
        let cover_type = match image_type_from_path(&self.cover.path)
        {
            Ok(t) => format!("{t:?}"),
            Err(e) =>
            {
                log::warn!("Could not determine cover type from path \"{}\": {e}", self.cover.path);
                "j".to_owned() // fallback to jpg
            }
        };

        // Parse media_id from String to u32
        let media_id: u32 = self.media_id.parse::<u32>().unwrap_or_else(|e|
        {
            log::warn!("Could not parse media_id \"{}\" to u32: {e}. Using 0.", self.media_id);
            0
        });


        // Insert/replace Hentai row
        let mut query: sqlx::query_builder::QueryBuilder<sqlx::Sqlite> = sqlx::query_builder::QueryBuilder::new(HENTAI_QUERY_STRING);
        query.push_values(std::iter::once(self), |mut builder, hentai|
        {
            builder
                .push_bind(hentai.id)
                .push_bind(&cover_type)
                .push_bind(media_id)
                .push_bind(hentai.num_favorites)
                .push_bind(hentai.num_pages)
                .push_bind(&page_types)
                .push_bind(hentai.scanlator.as_ref().and_then(|s| if s.is_empty() {None} else {Some(s)}))
                .push_bind(if hentai.title.english.is_empty() {None} else {Some(&hentai.title.english)})
                .push_bind(hentai.title.japanese.as_ref().and_then(|s| if s.is_empty() {None} else {Some(s)}))
                .push_bind(if hentai.title.pretty.is_empty() {None} else {Some(&hentai.title.pretty)})
                .push_bind(upload_date);
        });
        rows_affected += query
            .build()
            .persistent(false)
            .execute(&mut *db_tx).await?
            .rows_affected();

        // Insert/replace Tag rows
        if !self.tags.is_empty()
        {
            let mut query: sqlx::query_builder::QueryBuilder<sqlx::Sqlite> = sqlx::query_builder::QueryBuilder::new(TAG_QUERY_STRING);
            query.push_values(self.tags.iter(), |mut builder, tag|
            {
                builder
                    .push_bind(tag.id)
                    .push_bind(&tag.name)
                    .push_bind(&tag.r#type)
                    .push_bind(&tag.url);
            });
            rows_affected += query
                .build()
                .persistent(false)
                .execute(&mut *db_tx).await?
                .rows_affected();

            // Delete old Hentai_Tag entries for this hentai
            sqlx::query(format!("{HENTAI_TAG_QUERY1_STRING} ?;").as_str())
                .bind(self.id)
                .execute(&mut *db_tx).await?;

            // Insert new Hentai_Tag entries
            let mut query: sqlx::query_builder::QueryBuilder<sqlx::Sqlite> = sqlx::query_builder::QueryBuilder::new(HENTAI_TAG_QUERY2_STRING);
            query.push_values(self.tags.iter().map(|tag| (self.id, tag.id)), |mut builder, (hentai_id, tag_id)|
            {
                builder
                    .push_bind(hentai_id)
                    .push_bind(tag_id);
            });
            rows_affected += query
                .build()
                .persistent(false)
                .execute(&mut *db_tx).await?
                .rows_affected();
        }


        db_tx.commit().await?;
        return Ok(rows_affected);
    }
}