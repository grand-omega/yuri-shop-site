use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProductSummary {
    pub slug: String,
    pub name: String,
    pub subtitle: String,
    pub kind: String,
    pub price_cents: Option<i64>,
    pub currency: String,
    pub hero_image: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProductSpec {
    pub label: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProductImage {
    pub src: String,
    pub alt: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProductDetail {
    pub slug: String,
    pub name: String,
    pub subtitle: String,
    pub kind: String,
    pub price_cents: Option<i64>,
    pub currency: String,
    pub stock: Option<i64>,
    pub description_md: String,
    pub hero_image: String,
    pub specs: Vec<ProductSpec>,
    pub images: Vec<ProductImage>,
}

#[server]
pub async fn list_products() -> Result<Vec<ProductSummary>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    let rows = sqlx::query_as::<_, ProductSummary>(
        "SELECT slug, name, subtitle, kind, price_cents, currency, hero_image
         FROM products
         WHERE published = 1
         ORDER BY sort_order, name",
    )
    .fetch_all(&pool)
    .await?;
    Ok(rows)
}

#[server]
pub async fn get_product(slug: String) -> Result<Option<ProductDetail>, ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();

    let head: Option<(
        String,
        String,
        String,
        String,
        Option<i64>,
        String,
        Option<i64>,
        String,
        String,
    )> = sqlx::query_as(
        "SELECT slug, name, subtitle, kind, price_cents, currency, stock, description_md, hero_image
         FROM products
         WHERE slug = ?1 AND published = 1",
    )
    .bind(&slug)
    .fetch_optional(&pool)
    .await?;

    let Some((
        slug,
        name,
        subtitle,
        kind,
        price_cents,
        currency,
        stock,
        description_md,
        hero_image,
    )) = head
    else {
        return Ok(None);
    };

    let specs: Vec<ProductSpec> = sqlx::query_as(
        "SELECT label, value FROM product_specs WHERE product_slug = ?1 ORDER BY position",
    )
    .bind(&slug)
    .fetch_all(&pool)
    .await?;

    let images: Vec<ProductImage> = sqlx::query_as(
        "SELECT src, alt FROM product_images WHERE product_slug = ?1 ORDER BY position",
    )
    .bind(&slug)
    .fetch_all(&pool)
    .await?;

    Ok(Some(ProductDetail {
        slug,
        name,
        subtitle,
        kind,
        price_cents,
        currency,
        stock,
        description_md,
        hero_image,
        specs,
        images,
    }))
}
