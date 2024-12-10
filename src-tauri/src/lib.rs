use std::sync::mpsc::sync_channel;

use futures_util::stream::StreamExt;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationResponse},
    models::LocalModel,
    Ollama,
};
use serde::{Serialize, Serializer};
use tauri::{async_runtime::RwLock, ipc::Channel, AppHandle, Listener, Runtime, State};
use url::Url;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

type Connection = RwLock<Option<Ollama>>;
// type Model = RwLock<Option<String>>;

#[tauri::command]
async fn connect(
    connection: State<'_, Connection>,
    // model: State<'_, Model>,
    host: String,
) -> Result<(), ConnectError> {
    let url = Url::parse(&host)?;
    let mut connection = connection.write().await;
    // let mut model = model.write().await;
    connection.replace(Ollama::from_url(url));
    // model.take();
    Ok(())
}

#[derive(Debug, thiserror::Error, Serialize)]
enum ConnectError {
    #[error("invalid url: {0}")]
    InvalidUrl(
        #[serde(serialize_with = "serialize_url_parse_error")]
        #[from]
        #[source]
        url::ParseError,
    ),
}

#[tauri::command]
async fn disconnect(connection: State<'_, Connection>) -> Result<(), Infallible> {
    connection.write().await.take();
    Ok(())
}

#[tauri::command]
async fn list_models(
    connection: State<'_, Connection>,
) -> Result<Vec<LocalModel>, ListModelsError> {
    let connection_guard = connection.read().await;
    let connection = connection_guard
        .as_ref()
        .ok_or(ListModelsError::NoConnection)?;
    let models = connection.list_local_models().await?;
    Ok(models)
}

#[derive(Debug, thiserror::Error, Serialize)]
enum ListModelsError {
    #[error("no connection")]
    NoConnection,
    #[error(transparent)]
    Ollama(
        #[serde(serialize_with = "serialize_ollama_error")]
        #[from]
        ollama_rs::error::OllamaError,
    ),
}

#[tauri::command]
async fn translate<R: Runtime>(
    app: AppHandle<R>,
    input: String,
    input_format: String,
    output_format: String,
    connection: State<'_, Connection>,
    model: String,
    response_stream: Channel<TranslateResponseChunk>,
) -> Result<(), TranslateError> {
    let connection_guard = connection.read().await;
    let connection = connection_guard
        .as_ref()
        .ok_or(TranslateError::NoConnection)?;
    // let model_guard = model.read().await;
    // let model = model_guard.as_deref().ok_or(TranslateError::NoModel)?;

    let input_format = input_format.trim();
    let input_format = (!input_format.is_empty()).then_some(input_format);

    let prompt = format!(
        "Translate this input of format {} to format {output_format}. Don't explain anything, be concise, write only the translation.\nInput:\n{input}",
        input_format.unwrap_or("auto")
    );

    let request = GenerationRequest::new(model.to_owned(), prompt);

    let (tx, rx) = sync_channel(1);
    app.listen("stop-translation", move |_| {
        _ = tx.send(());
    });
    let mut stream = connection.generate_stream(request).await?;
    if rx.try_recv().is_ok() {
        response_stream.send(TranslateResponseChunk::EndOfStream)?;
        return Ok(());
    }
    while let Some(result) = stream.next().await {
        let responses = result?;
        let stop = rx.try_recv().is_ok();
        for response in responses {
            response_stream.send(response.into())?;
        }
        if stop {
            break;
        }
    }
    response_stream.send(TranslateResponseChunk::EndOfStream)?;

    Ok(())
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "tag")]
enum TranslateResponseChunk {
    Response(GenerationResponse),
    EndOfStream,
}

impl From<GenerationResponse> for TranslateResponseChunk {
    fn from(response: GenerationResponse) -> Self {
        Self::Response(response)
    }
}

#[derive(Debug, thiserror::Error, Serialize)]
enum TranslateError {
    #[error("no connection")]
    NoConnection,
    // #[error("model not selected")]
    // NoModel,
    #[error(transparent)]
    Ollama(
        #[serde(serialize_with = "serialize_ollama_error")]
        #[from]
        ollama_rs::error::OllamaError,
    ),
    #[error(transparent)]
    Tauri(
        #[serde(serialize_with = "serialize_tauri_error")]
        #[from]
        tauri::Error,
    ),
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(Connection::default())
        // .manage(Model::default())
        .invoke_handler(tauri::generate_handler![
            connect,
            disconnect,
            list_models,
            translate
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn serialize_ollama_error<S: Serializer>(
    error: &ollama_rs::error::OllamaError,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&format_args!("{error:?}"))
}

fn serialize_url_parse_error<S: Serializer>(
    error: &url::ParseError,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&format_args!("{error}"))
}

fn serialize_tauri_error<S: Serializer>(
    error: &tauri::Error,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.collect_str(&format_args!("{error}"))
}

#[derive(Debug, thiserror::Error, Serialize)]
enum Infallible {}
