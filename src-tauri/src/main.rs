use diary_core::{db::SortOrder, Config, DiaryDB, Entry, Pagination};
use tauri::Manager;
use tokio::sync::mpsc;

// Channel message types remain the same
#[derive(Debug)]
pub enum DBRequest {
    CreateEntry {
        content: String,
        pinned: bool,
        response_tx: mpsc::Sender<Result<Entry, String>>,
    },
    ReadEntry {
        id: i64,
        response_tx: mpsc::Sender<Result<Entry, String>>,
    },
    ReadEntries {
        page: Option<i64>,
        per_page: Option<i64>,
        sort: Option<SortOrder>,
        pinned: Option<bool>,
        substring: Option<String>,
        response_tx: mpsc::Sender<Result<Pagination, String>>,
    },
    UpdateEntry {
        id: i64,
        content: Option<String>,
        pinned: Option<bool>,
        response_tx: mpsc::Sender<Result<Entry, String>>,
    },
    DeleteEntry {
        id: i64,
        response_tx: mpsc::Sender<Result<(), String>>,
    },
}

// DiaryState remains the same
pub struct DiaryState {
    request_tx: mpsc::Sender<DBRequest>,
}

impl DiaryState {
    pub async fn new(config: Config) -> Result<Self, String> {
        // Implementation remains the same
        let (request_tx, mut request_rx) = mpsc::channel::<DBRequest>(32);
        let db = DiaryDB::new(&config.db_url)
            .await
            .map_err(|e| e.to_string())?;

        tokio::spawn(async move {
            while let Some(request) = request_rx.recv().await {
                match request {
                    DBRequest::CreateEntry {
                        content,
                        pinned,
                        response_tx,
                    } => {
                        let result = db
                            .db
                            .create_entry(content, pinned)
                            .await
                            .map_err(|e| e.to_string());
                        let _ = response_tx.send(result).await;
                    }
                    DBRequest::ReadEntries {
                        page,
                        per_page,
                        sort,
                        pinned,
                        substring,
                        response_tx,
                    } => {
                        let result = db
                            .db
                            .read_entries_with_pagination(page, per_page, sort, pinned, substring)
                            .await
                            .map_err(|e| e.to_string());
                        let _ = response_tx.send(result).await;
                    }
                    DBRequest::ReadEntry { id, response_tx } => {
                        let result = db.db.read_entry(id).await.map_err(|e| e.to_string());
                        let _ = response_tx.send(result).await;
                    }
                    DBRequest::UpdateEntry {
                        id,
                        content,
                        pinned,
                        response_tx,
                    } => {
                        let result = db
                            .db
                            .update_entry(id, content, pinned)
                            .await
                            .map_err(|e| e.to_string());
                        let _ = response_tx.send(result).await;
                    }
                    DBRequest::DeleteEntry { id, response_tx } => {
                        let result = db.db.delete_entry(id).await.map_err(|e| e.to_string());
                        let _ = response_tx.send(result).await;
                    }
                }
            }
        });

        Ok(Self { request_tx })
    }
}

// Modified CommandResponse
#[derive(Debug, serde::Serialize)]
pub struct CommandResponse<T> {
    data: Option<T>,
    error: Option<String>,
}

impl<T> CommandResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            error: None,
        }
    }

    pub fn error(err: impl std::fmt::Display) -> Self {
        Self {
            data: None,
            error: Some(err.to_string()),
        }
    }
}

// Modified Tauri command handlers to return Result
#[tauri::command]
async fn create_entry(
    state: tauri::State<'_, DiaryState>,
    content: String,
    pinned: bool,
) -> Result<CommandResponse<Entry>, String> {
    let (response_tx, mut response_rx) = mpsc::channel(1);

    state
        .request_tx
        .send(DBRequest::CreateEntry {
            content,
            pinned,
            response_tx,
        })
        .await
        .map_err(|e| e.to_string())?;

    match response_rx.recv().await {
        Some(Ok(entry)) => Ok(CommandResponse::success(entry)),
        Some(Err(e)) => Ok(CommandResponse::error(e)),
        None => Ok(CommandResponse::error("Failed to receive response")),
    }
}

#[tauri::command]
async fn read_entry(
    state: tauri::State<'_, DiaryState>,
    id: i64,
) -> Result<CommandResponse<Entry>, String> {
    let (response_tx, mut response_rx) = mpsc::channel(1);

    state
        .request_tx
        .send(DBRequest::ReadEntry { id, response_tx })
        .await
        .map_err(|e| e.to_string())?;

    match response_rx.recv().await {
        Some(Ok(entry)) => Ok(CommandResponse::success(entry)),
        Some(Err(e)) => Ok(CommandResponse::error(e)),
        None => Ok(CommandResponse::error("Failed to receive response")),
    }
}

#[tauri::command]
async fn read_entries(
    state: tauri::State<'_, DiaryState>,
    page: Option<i64>,
    per_page: Option<i64>,
    sort: Option<SortOrder>,
    pinned: Option<bool>,
    substring: Option<String>,
) -> Result<CommandResponse<Pagination>, String> {
    let (response_tx, mut response_rx) = mpsc::channel(1);

    state
        .request_tx
        .send(DBRequest::ReadEntries {
            page,
            per_page,
            sort,
            pinned,
            substring,
            response_tx,
        })
        .await
        .map_err(|e| e.to_string())?;

    match response_rx.recv().await {
        Some(Ok(pagination)) => Ok(CommandResponse::success(pagination)),
        Some(Err(e)) => Ok(CommandResponse::error(e)),
        None => Ok(CommandResponse::error("Failed to receive response")),
    }
}

#[tauri::command]
async fn update_entry(
    state: tauri::State<'_, DiaryState>,
    id: i64,
    content: Option<String>,
    pinned: Option<bool>,
) -> Result<CommandResponse<Entry>, String> {
    let (response_tx, mut response_rx) = mpsc::channel(1);

    state
        .request_tx
        .send(DBRequest::UpdateEntry {
            id,
            content,
            pinned,
            response_tx,
        })
        .await
        .map_err(|e| e.to_string())?;

    match response_rx.recv().await {
        Some(Ok(entry)) => Ok(CommandResponse::success(entry)),
        Some(Err(e)) => Ok(CommandResponse::error(e)),
        None => Ok(CommandResponse::error("Failed to receive response")),
    }
}

#[tauri::command]
async fn delete_entry(
    state: tauri::State<'_, DiaryState>,
    id: i64,
) -> Result<CommandResponse<()>, String> {
    let (response_tx, mut response_rx) = mpsc::channel(1);

    state
        .request_tx
        .send(DBRequest::DeleteEntry { id, response_tx })
        .await
        .map_err(|e| e.to_string())?;

    match response_rx.recv().await {
        Some(Ok(())) => Ok(CommandResponse::success(())),
        Some(Err(e)) => Ok(CommandResponse::error(e)),
        None => Ok(CommandResponse::error("Failed to receive response")),
    }
}

#[tokio::main]
async fn main() {
    let config = Config::from_file("config.ini").unwrap_or_else(|_| Config {
        db_url: String::from("sqlite://diary.db"),
    });

    let diary_state = DiaryState::new(config)
        .await
        .expect("Failed to create diary state");

    tauri::Builder::default()
        .setup(|app| {
            // Get path to config directory
            let config_path = app
                .path()
                .app_config_dir()
                .expect("Failed to get config dir");
            println!("Config directory: {:?}", config_path);

            // Or get path to app data directory
            let data_path = app.path().app_data_dir().expect("Failed to get data dir");
            println!("Data directory: {:?}", data_path);

            Ok(())
        })
        .manage(diary_state)
        .invoke_handler(tauri::generate_handler![
            create_entry,
            read_entry,
            read_entries,
            update_entry,
            delete_entry,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
