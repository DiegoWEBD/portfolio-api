use serde::{Deserialize, Serialize};

pub mod document_repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    id: Option<i32>,
    title: String,
    description: String,
    download_url: String
}

impl Document {

    pub fn new(id: Option<i32>, title: String, description: String, download_url: String) -> Self {
        Document {
            id,
            title,
            description,
            download_url
        }
    }

    pub fn get_id(&self) -> Option<i32> {
        self.id
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_download_url(&self) -> String {
        self.download_url.clone()
    }
}