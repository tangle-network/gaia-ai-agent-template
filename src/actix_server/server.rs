use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use color_eyre::Result;
use gadget_sdk::info;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::{
    gaia_client::{APIError, GaiaNodeClient},
    types::{ChatRequest, CreateImageRequest, EditImageRequest},
};

struct AppState {
    gaia_client: Arc<Mutex<GaiaNodeClient>>,
    service_id: u64,
}

async fn handle_gaia_request<T, F, Fut, R>(
    app_state: web::Data<AppState>,
    request: web::Json<T>,
    operation: F,
) -> impl Responder
where
    F: FnOnce(Arc<Mutex<GaiaNodeClient>>, T) -> Fut,
    Fut: std::future::Future<Output = Result<R, APIError>>,
    R: serde::Serialize,
{
    let gaia_client = app_state.gaia_client.clone();
    match operation(gaia_client, request.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(e) => HttpResponse::InternalServerError().json(format!("Error: {}", e)),
    }
}

async fn chat(
    app_state: web::Data<AppState>,
    chat_request: web::Json<ChatRequest>,
) -> impl Responder {
    handle_gaia_request(app_state, chat_request, |client, request| async move {
        let client = client.lock().await;
        client.chat(request.messages).await
    })
    .await
}

async fn analyze_image(
    app_state: web::Data<AppState>,
    image_url: web::Json<String>,
) -> impl Responder {
    handle_gaia_request(app_state, image_url, |client, url| async move {
        let client = client.lock().await;
        client.analyze_image(url).await
    })
    .await
}

async fn create_image(
    app_state: web::Data<AppState>,
    create_request: web::Json<CreateImageRequest>,
) -> impl Responder {
    handle_gaia_request(app_state, create_request, |client, request| async move {
        let client = client.lock().await;
        client
            .create_image(
                request.prompt,
                request.n,
                request.quality,
                request.size,
                request.style,
            )
            .await
    })
    .await
}

async fn edit_image(
    app_state: web::Data<AppState>,
    edit_request: web::Json<EditImageRequest>,
) -> impl Responder {
    handle_gaia_request(app_state, edit_request, |client, request| async move {
        let client = client.lock().await;
        client
            .edit_image(
                request.image_path,
                request.prompt,
                request.mask_path,
                request.n,
                request.size,
            )
            .await
    })
    .await
}

pub async fn run_server(service_id: u64, model: String) -> Result<()> {
    let app_state = web::Data::new(AppState {
        gaia_client: Arc::new(Mutex::new(GaiaNodeClient::new(
            "https://YOUR-NODE-ID.us.gaianet.network/v1".to_string(),
            "".to_string(),
            model,
        ))),
        service_id,
    });

    info!(
        "Starting server with base URL: {} and service ID: {}",
        app_state.gaia_client.lock().await.base_url,
        app_state.service_id
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/chat", web::post().to(chat))
            .route("/analyze_image", web::post().to(analyze_image))
            .route("/create_image", web::post().to(create_image))
            .route("/edit_image", web::post().to(edit_image))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
