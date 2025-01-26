use diary_core::{db::SortOrder, Config, DiaryDB, Entry, Pagination};
use std::fs::File;
use std::io::{self};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{path::BaseDirectory, Manager};
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
    DumpEntries {
        response_tx: mpsc::Sender<Result<(), String>>,
    },
    Shutdown,
}

// DiaryState remains the same
pub struct DiaryState {
    request_tx: mpsc::Sender<DBRequest>,
    shutdown_tx: mpsc::Sender<()>,
    is_shutting_down: Arc<AtomicBool>,
}

impl DiaryState {
    pub async fn new(config: Config) -> Result<Self, String> {
        // Implementation remains the same
        let (request_tx, mut request_rx) = mpsc::channel::<DBRequest>(32);
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
        let is_shutting_down = Arc::new(AtomicBool::new(false));
        let db = DiaryDB::new(&config.db_url)
            .await
            .map_err(|e| e.to_string())?;

        tokio::spawn(async move {
            let mut running = true;

            tokio::select! {
                            _ = async {
                                loop {
                                    if !running {
                                        break;
                                    }

                                    match request_rx.recv().await {
                                        Some(DBRequest::Shutdown) => {
                                            running = false;
                                            db.db.close().await;
                                        },
                                        Some(DBRequest::CreateEntry {
                                            content,
                                            pinned,
                                            response_tx,
                                        }) => {
                                            let result = db
                                                .db
                                                .create_entry(content, pinned)
                                                .await
                                                .map_err(|e| e.to_string());
                                            let _ = response_tx.send(result).await;
                                        }
                                        Some(DBRequest::ReadEntries {
                                            page,
                                            per_page,
                                            sort,
                                            pinned,
                                            substring,
                                            response_tx,
                                        }) => {
                                            let result = db
                                                .db
                                                .read_entries_with_pagination(page, per_page, sort, pinned, substring)
                                                .await
                                                .map_err(|e| e.to_string());
                                            let _ = response_tx.send(result).await;
                                        }
                                        Some(DBRequest::ReadEntry { id, response_tx }) => {
                                            let result = db.db.read_entry(id).await.map_err(|e| e.to_string());
                                            let _ = response_tx.send(result).await;
                                        }
                                        Some(DBRequest::UpdateEntry {
                                            id,
                                            content,
                                            pinned,
                                            response_tx,
                                        }) => {
                                            let result = db
                                                .db
                                                .update_entry(id, content, pinned)
                                                .await
                                                .map_err(|e| e.to_string());
                                            let _ = response_tx.send(result).await;
                                        }
                                        Some(DBRequest::DeleteEntry { id, response_tx }) => {
                                            let result = db.db.delete_entry(id).await.map_err(|e| e.to_string());
                                            let _ = response_tx.send(result).await;
                                        },
                                        Some(DBRequest::DumpEntries {response_tx}) => {
                                            let result = db.db.dump_entries(None).await.map_err(|e| e.to_string());
                                            let _ = response_tx.send(result).await;
                                        },
                                        None => break,
                                    }
                                }} => {},
                                _ = shutdown_rx.recv() => {
            db.db.close().await;
                                }
                            }
        });

        Ok(Self {
            request_tx,
            shutdown_tx,
            is_shutting_down,
        })
    }

    pub async fn shutdown(&self) -> Result<(), String> {
        self.request_tx
            .send(DBRequest::Shutdown)
            .await
            .map_err(|e| e.to_string())
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

#[tauri::command]
async fn dump_entries(state: tauri::State<'_, DiaryState>) -> Result<CommandResponse<()>, String> {
    let (response_tx, mut response_rx) = mpsc::channel(1);

    state
        .request_tx
        .send(DBRequest::DumpEntries { response_tx })
        .await
        .map_err(|e| e.to_string())?;

    match response_rx.recv().await {
        Some(Ok(())) => Ok(CommandResponse::success(())),
        Some(Err(e)) => Ok(CommandResponse::error(e)),
        None => Ok(CommandResponse::error("Failed to receive response")),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        // .manage(diary_state)
        .setup(move |app| {
            let config_path = std::env::current_exe()
                .unwrap()
                .parent()
                .expect("Failed to get cwd")
                .join("config.ini");

            println!("[Setup] Config path: {:?}", config_path);

            let mut config_file = File::options()
                .read(true)
                .write(true)
                .create(true)
                .open(&config_path)
                .map_err(|e| {
                    println!("Failed to open config file: {:?}", e);
                    e
                })
                .unwrap();

            if config_file.metadata().unwrap().len() == 0 {
                let resource_path = app
                    .path()
                    .resolve("resources/config.ini.sample", BaseDirectory::Resource)
                    .expect("Failed to resolve resource path");
                println!("[Setup] resource path {:?}", resource_path);
                let mut config_sample_file = File::options()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(&resource_path)
                    .map_err(|e| {
                        println!("[Setup] Failed to open sample config file: {:?}", e);
                        e
                    })
                    .unwrap();

                io::copy(&mut config_sample_file, &mut config_file)
                    .expect("[Setup] Failed to copy config sample into working directory.");
            }

            let diary_state = futures::executor::block_on(async {
                DiaryState::new(Config::from_file(config_path).unwrap())
                    .await
                    .unwrap()
            });

            app.manage(diary_state);

            Ok(())
        })
        // .manage(diary_state)
        .on_window_event(move |window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // Get what we need from state first
                let tx = {
                    let state = window.state::<DiaryState>();
                    if state.is_shutting_down.load(Ordering::SeqCst) {
                        println!("Already shutting down, proceeding with close");
                        return;
                    }
                    state.is_shutting_down.store(true, Ordering::SeqCst);
                    state.request_tx.clone() // Clone the sender
                };

                println!("Starting shutdown process");

                // Now use the cloned sender
                let window = window.clone();
                futures::executor::block_on(async move {
                    println!("Sending shutdown request");
                    if let Err(e) = tx.send(DBRequest::Shutdown).await {
                        eprintln!("Failed to send shutdown request: {}", e);
                    }
                    println!("Shutdown request sent, closing window");
                    let _ = window.close();
                });

                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            create_entry,
            read_entry,
            read_entries,
            update_entry,
            delete_entry,
            dump_entries,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
