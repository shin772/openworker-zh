// STT stub – 移除 whisper-rs 依赖，保留完整公开 API
// 所有方法使用 &self（内部 Mutex），编译无需 CMake/LLVM

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub const DEFAULT_MODEL_FILE: &str = "ggml-base.en.bin";
pub const DEFAULT_MODEL_URL: &str = "";
pub const DEFAULT_MODEL_BYTES: u64 = 0;
pub const DEFAULT_MODEL_SHA256: &str = "";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictationStatus {
    pub is_listening: bool,
    pub is_downloading: bool,
    pub error: Option<String>,
    pub recording: bool,
    pub model_installed: bool,
    pub model_verified: bool,
    pub test_passed: bool,
    pub download_in_progress: bool,
    pub model_name: &'static str,
    pub model_bytes: u64,
}

impl Default for DictationStatus {
    fn default() -> Self {
        Self {
            is_listening: false,
            is_downloading: false,
            error: Some("本地语音输入未编译（STT stub）".into()),
            recording: false,
            model_installed: false,
            model_verified: false,
            test_passed: false,
            download_in_progress: false,
            model_name: "",
            model_bytes: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug)]
pub struct Dictation {
    status: Mutex<DictationStatus>,
}

impl Dictation {
    pub fn new() -> Self {
        Self {
            status: Mutex::new(DictationStatus::default()),
        }
    }

    pub fn status(&self) -> DictationStatus {
        self.status.lock().unwrap().clone()
    }

    pub fn start(&self) -> Result<(), String> {
        Err("STT not available (stub)".into())
    }

    pub fn stop(&self) {}

    pub fn transcribe(&self) -> Result<String, String> {
        Err("STT not available".into())
    }

    pub fn stop_and_transcribe(&self) -> Result<String, String> {
        Err("STT not available".into())
    }

    pub fn install_default_model_with_progress<F>(&self, _on_progress: F) -> Result<(), String>
    where
        F: Fn(DownloadProgress) + Send + 'static,
    {
        Err("STT not available".into())
    }

    pub fn verify_default_model(&self) -> Result<(), String> {
        Err("STT not available".into())
    }

    pub fn cancel(&self) {}

    pub fn cancel_model_download(&self) {}

    pub fn mark_test_passed(&self) -> Result<(), String> {
        Err("STT not available".into())
    }

    pub fn delete_default_model(&self) -> Result<(), String> {
        Err("STT not available".into())
    }

    pub fn input_level(&self) -> f32 {
        0.0
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
