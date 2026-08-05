// STT stub – 移除 whisper-rs 依赖，保留公开 API 签名
// 所有方法返回 "not available" 状态，编译无需 CMake/LLVM

use serde::{Deserialize, Serialize};

pub const DEFAULT_MODEL_FILE: &str = "ggml-base.en.bin";
pub const DEFAULT_MODEL_URL: &str = "";
pub const DEFAULT_MODEL_BYTES: u64 = 0;
pub const DEFAULT_MODEL_SHA256: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictationStatus {
    pub is_listening: bool,
    pub is_downloading: bool,
    pub error: Option<String>,
}

impl Default for DictationStatus {
    fn default() -> Self {
        Self {
            is_listening: false,
            is_downloading: false,
            error: Some("本地语音输入未编译（STT stub）".into()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug)]
pub struct Dictation;

impl Dictation {
    pub fn new() -> Result<Self, String> {
        Err("STT not available (stub build)".into())
    }

    pub fn status(&self) -> DictationStatus {
        DictationStatus::default()
    }

    pub fn start(&mut self) -> Result<(), String> {
        Err("STT not available".into())
    }

    pub fn stop(&mut self) {
    }

    pub fn transcribe(&mut self) -> Result<String, String> {
        Err("STT not available".into())
    }

    pub fn download_progress(&self) -> DownloadProgress {
        DownloadProgress {
            downloaded_bytes: 0,
            total_bytes: 1,
        }
    }

    pub fn is_model_downloaded() -> bool {
        false
    }

    pub fn download_model<F>(_on_progress: F) -> Result<(), String>
    where
        F: Fn(DownloadProgress) + Send + 'static,
    {
        Err("STT not available".into())
    }
}
