use openai_dive::v1::resources::{
    chat::ChatMessage,
    image::{ImageQuality, ImageSize, ImageStyle},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatRequest {
    pub messages: Vec<ChatMessage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatResponse {
    pub response: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateImageRequest {
    pub prompt: String,
    pub n: u32,
    pub quality: ImageQuality,
    pub size: ImageSize,
    pub style: ImageStyle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditImageRequest {
    pub image_path: String,
    pub prompt: String,
    pub mask_path: Option<String>,
    pub n: u32,
    pub size: ImageSize,
}
