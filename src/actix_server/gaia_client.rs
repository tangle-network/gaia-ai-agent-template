use openai_dive::v1::{
    api::Client,
    resources::{
        chat::{
            ChatCompletionParametersBuilder, ChatCompletionParametersBuilderError,
            ChatCompletionResponse, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
            ImageUrl, ImageUrlType,
        },
        image::{
            CreateImageParametersBuilder, CreateImageParametersBuilderError,
            EditImageParametersBuilder, EditImageParametersBuilderError, ImageQuality,
            ImageResponse, ImageSize, ImageStyle, ResponseFormat,
        },
        shared::FileUpload,
    },
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("OpenAiDive error: {0}")]
    OpenAiDiveError(#[from] openai_dive::v1::error::APIError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Chat completion error: {0}")]
    ChatCompletionError(#[from] ChatCompletionParametersBuilderError),

    #[error("Image creation error: {0}")]
    ImageCreationError(#[from] CreateImageParametersBuilderError),

    #[error("Image edit error: {0}")]
    ImageEditError(#[from] EditImageParametersBuilderError),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("IO error: {0}")]
    IOError(String),
}

// GaiaNodeClient implementation using openai_dive-like structure
pub struct GaiaNodeClient {
    pub base_url: String,
    pub api_key: String,
    pub current_model: String,
}

impl GaiaNodeClient {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        Self {
            base_url,
            api_key,
            current_model: model,
        }
    }

    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
    ) -> Result<ChatCompletionResponse, APIError> {
        let client = Client::new_with_base(&self.base_url, self.api_key.clone());

        let parameters = ChatCompletionParametersBuilder::default()
            .model(self.current_model.clone())
            .messages(messages)
            .response_format(ChatCompletionResponseFormat::Text)
            .build()?;

        let result = client.chat().create(parameters).await?;

        Ok(result)
    }

    pub async fn analyze_image(
        &self,
        image_url: String,
    ) -> Result<ChatCompletionResponse, APIError> {
        let client = Client::new_with_base(&self.base_url, self.api_key.clone());

        let parameters = ChatCompletionParametersBuilder::default()
            .model(self.current_model.clone())
            .messages(vec![
                ChatMessage::User {
                    content: ChatMessageContent::Text("What is in this image?".to_string()),
                    name: None,
                },
                ChatMessage::User {
                    content: ChatMessageContent::ImageUrl(vec![ImageUrl {
                        r#type: "image_url".to_string(),
                        text: None,
                        image_url: ImageUrlType {
                            url: image_url,
                            detail: None,
                        },
                    }]),
                    name: None,
                },
            ])
            .build()?;

        let result = client.chat().create(parameters).await?;

        Ok(result)
    }

    pub async fn create_image(
        &self,
        prompt: String,
        n: u32,
        quality: ImageQuality,
        size: ImageSize,
        style: ImageStyle,
    ) -> Result<ImageResponse, APIError> {
        let client = Client::new_with_base(&self.base_url, self.api_key.clone());

        let parameters = CreateImageParametersBuilder::default()
            .prompt(prompt)
            .model(self.current_model.clone())
            .n(n)
            .quality(quality)
            .response_format(ResponseFormat::Url)
            .size(size)
            .style(style)
            .build()?;

        let result = client.images().create(parameters).await?;

        Ok(result)
    }

    pub async fn edit_image(
        &self,
        image_path: String,
        prompt: String,
        mask_path: Option<String>,
        n: u32,
        size: ImageSize,
    ) -> Result<ImageResponse, APIError> {
        let client = Client::new_with_base(&self.base_url, self.api_key.clone());

        let image_path = download_or_verify_file(&image_path, "image").await?;
        let mask_path = if let Some(mask) = mask_path {
            Some(download_or_verify_file(&mask, "mask").await?)
        } else {
            None
        };

        let mut parameters_builder = EditImageParametersBuilder::default()
            .image(FileUpload::File(image_path))
            .prompt(prompt)
            .n(n)
            .size(size)
            .clone();

        if let Some(mask) = mask_path {
            parameters_builder = parameters_builder.mask(FileUpload::File(mask)).clone();
        }

        let parameters = parameters_builder.build()?;

        let result = client.images().edit(parameters).await?;

        Ok(result)
    }
}

pub async fn download_or_verify_file(path: &str, prefix: &str) -> Result<String, APIError> {
    if path.starts_with("http://") || path.starts_with("https://") {
        let response = reqwest::get(path).await?;
        let bytes = response.bytes().await?;
        let temp_dir = std::env::temp_dir();
        let file_name = format!(
            "temp_{}_{}_{}",
            prefix,
            uuid::Uuid::new_v4(),
            std::path::Path::new(path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );
        let temp_path = temp_dir.join(file_name);
        tokio::fs::write(&temp_path, &bytes)
            .await
            .map_err(|e| APIError::IOError(e.to_string()))?;
        Ok(temp_path.to_string_lossy().into_owned())
    } else {
        let path = std::path::Path::new(path);
        if !path.exists() {
            return Err(APIError::InvalidRequest(format!(
                "{} file not found: {}",
                prefix,
                path.display()
            )));
        }
        Ok(path.to_string_lossy().into_owned())
    }
}
