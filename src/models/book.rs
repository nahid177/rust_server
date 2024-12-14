use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub facebook_url: String,
    pub whatsapp_url: String,
    pub images: Vec<String>,  // Store the image URLs or paths
    pub user_contact_number: String,
    pub details: String,
    pub category: String,
    #[serde(rename = "createdBy")]
    pub created_by: ObjectId,
    pub created_at: Option<DateTime<Utc>>,
    pub approved: bool,
}
