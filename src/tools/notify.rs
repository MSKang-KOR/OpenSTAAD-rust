use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use notify::event::RenameMode;
use notify::recommended_watcher;
use notify::{Event, EventKind, RecursiveMode, Result, Watcher, event::ModifyKind};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Runtime};

/// 백그라운드에서 파일 감시를 시작하는 함수
/// JoinHandle을 반환하여 필요시 스레드를 제어할 수 있음
pub fn watch_file_background(path: &PathBuf, app_handle: AppHandle) -> thread::JoinHandle<()> {
    let path_str = path.to_str().unwrap().to_string();
    thread::spawn(move || {
        let (tx, rx) = mpsc::channel::<Result<Event>>();
        let file_cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
        let last_activity: Arc<Mutex<Instant>> = Arc::new(Mutex::new(Instant::now()));

        match fs::read_to_string(&path_str) {
            Ok(initial_content) => {
                let mut cache = file_cache.lock().unwrap();
                cache.insert((&path_str).to_string(), initial_content);
            }
            Err(_) => {
                // 파일이 없어도 무시
            }
        }

        let mut watcher = notify::recommended_watcher(tx).unwrap();
        let file_path_obj = Path::new(&path_str);
        let watch_path = if let Some(parent) = file_path_obj.parent() {
            parent
        } else {
            Path::new(".")
        };

        let _ = watcher.watch(watch_path, RecursiveMode::NonRecursive);

        loop {
            match rx.recv_timeout(Duration::from_secs(600)) {
                Ok(event) => {
                    if handle_file_event(
                        event.unwrap(),
                        &file_cache,
                        &path_str,
                        &last_activity,
                        &app_handle,
                    ) {
                        println!("End::End of Analysis.");
                        break; // "End Of Analysis" 감지로 종료
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    let last_time = last_activity.lock().unwrap();
                    let elapsed = last_time.elapsed();
                    if elapsed >= Duration::from_secs(10) {
                        println!("End::Time out.");
                        break;
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    println!("End::Disconnected.");
                    break;
                }
            }
        }
    })
}

/// 파일 변경 이벤트를 처리하는 함수
/// 종료 조건이 감지되면 true를 반환
fn handle_file_event(
    event: Event,
    file_cache: &Arc<Mutex<HashMap<String, String>>>,
    target_file: &str,
    last_activity: &Arc<Mutex<Instant>>,
    app: &AppHandle,
) -> bool {
    // 타겟 파일과 관련된 이벤트만 처리
    let target_path = Path::new(target_file);
    let is_target_file = event.paths.iter().any(|path| path == target_path);

    if !is_target_file {
        return false; // 타겟 파일이 아니면 무시
    }

    // 마지막 활동 시간 업데이트
    {
        let mut last_time = last_activity.lock().unwrap();
        *last_time = Instant::now();
    }

    match event.kind {
        EventKind::Create(_) => {
            for path in event.paths {
                if path == target_path {
                    if check_and_print_changes(&path, file_cache, app) {
                        return true;
                    }
                }
            }
        }
        EventKind::Modify(_) => {
            for path in event.paths {
                if path == target_path {
                    if check_and_print_changes(&path, file_cache, app) {
                        return true;
                    }
                }
            }
        }
        EventKind::Remove(_) => {
            for path in event.paths {
                if path == target_path {
                    // 캐시에서도 제거
                    let mut cache = file_cache.lock().unwrap();
                    cache.remove(&path.to_string_lossy().to_string());
                }
            }
        }
        _ => {
            // 기타 이벤트는 무시
        }
    }
    false
}

/// 파일의 현재 내용을 읽어서 변경사항을 확인하는 함수
/// 종료 조건이 감지되면 true를 반환
fn check_and_print_changes(
    file_path: &Path,
    file_cache: &Arc<Mutex<HashMap<String, String>>>,
    app: &AppHandle,
) -> bool {
    let path_str = file_path.to_string_lossy().to_string();

    match fs::read_to_string(file_path) {
        Ok(new_content) => {
            let mut cache = file_cache.lock().unwrap();

            if let Some(old_content) = cache.get(&path_str) {
                if old_content != &new_content {
                    let is_emit = emit_progress(old_content, &new_content, app);
                    cache.insert(path_str, new_content);
                    return is_emit;
                }
            } else {
                cache.insert(path_str, new_content);
            }
        }
        Err(_) => {
            // 에러 메시지 제거
        }
    }
    false
}

/// 두 텍스트 간의 차이점을 찾아서 출력하는 함수
/// 종료 조건이 감지되면 true를 반환
fn emit_progress(old_content: &str, new_content: &str, app: &AppHandle) -> bool {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();

    let max_lines = old_lines.len().max(new_lines.len());
    let mut is_emit = false;

    for i in 0..max_lines {
        let old_line = old_lines.get(i).unwrap_or(&"");
        let new_line = new_lines.get(i).unwrap_or(&"");

        if old_line != new_line {
            if i < old_lines.len() && i < new_lines.len() {
                // 라인이 변경됨
                if new_line.to_lowercase().contains("end of analysis") {
                    is_emit = true;
                }
            } else if i >= old_lines.len() {
                // 새 라인이 추가됨
                let _ = app
                    .emit("staad_analysis_progress", new_line)
                    .map_err(|e| e.to_string());
                if new_line.to_lowercase().contains("end of analysis") {
                    is_emit = true;
                }
            }
        }
    }
    is_emit
}

// 프론트엔드로 보낼 이벤트 데이터 구조체
#[derive(Clone, Serialize)]
struct FileNamesPayload {
    // kind: String,
    data: Vec<String>,
}

// 파일 검사 및 필터링 로직을 담은 헬퍼 함수
pub fn get_base_std_files(dir_path: &Path) -> Result<Vec<String>> {
    let mut filtered_files = Vec::new();

    // 디렉토리를 재귀적이지 않게 읽습니다 (자식 디렉토리는 검사하지 않음)
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                // 1. 확장자가 .STD 인지 확인 (대소문자 구분 없이)
                let is_std = path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map_or(false, |ext| ext.eq_ignore_ascii_case("STD"));

                // 2. 파일명에 "_"이 포함되지 않았는지 확인
                let no_underscore = !file_name.contains('_');
                let no_cloned = !file_name.contains("복사본");
                let no_init = !file_name.contains("INIT");

                if is_std && no_underscore && no_cloned && no_init {
                    // 조건 만족 시, 파일의 이름(String)을 목록에 추가
                    // 필요하다면 path.display().to_string() 등으로 전체 경로를 보낼 수도 있습니다.
                    filtered_files.push(file_name.to_string());
                }
            }
        }
    }

    Ok(filtered_files)
}

pub fn start_file_watcher<R: Runtime>(
    app_handle: AppHandle<R>,
    path: PathBuf,
    mut shutdown_rx: tokio::sync::oneshot::Receiver<()>,
) -> Result<tauri::async_runtime::JoinHandle<()>> {
    // 💡 tokio::sync::mpsc에서 온 mpsc::channel을 사용합니다.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Event>(100);
    let dir_to_watch = path.clone();

    // Watcher 설정
    let mut watcher = recommended_watcher(move |res: notify::Result<Event>| {
        if let Ok(event) = res {
            // tx.blocking_send는 tokio::sync::mpsc::Sender가 제공합니다.
            if tx.blocking_send(event).is_err() {
                eprintln!("파일 변경 이벤트 전송 실패: 수신자 드롭됨");
            }
        } else if let Err(e) = res {
            eprintln!("파일 감시 오류: {:?}", e);
        }
    })?;

    // 🚨 초기 파일 목록 전송 로직 (생략된 함수 get_base_staad_files 사용)

    // 감시 시작 (재귀적 감시)
    watcher.watch(&path, RecursiveMode::Recursive)?;
    let handle = tauri::async_runtime::spawn(async move {
        let _watcher_guard = watcher;

        loop {
            // 이벤트 수신과 종료 신호 수신을 동시에 대기
            tokio::select! {
                // 1. 파일 변경 이벤트가 도착함 (rx.recv()는 tokio::sync::mpsc::Receiver의 메서드)
                event = rx.recv() => {
                    if let Some(event) = event {
                        let mut should_rescan = false;
                        let is_relevant_kind = matches!(event.kind, EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(ModifyKind::Name(RenameMode::To)));
                        println!("{:#?}: {:#?}", event.kind, event.paths);
                        if is_relevant_kind {
                            should_rescan = event.paths.iter().any(|p| {
                                // 파일 확장자가 .stt 또는 .std 인지 확인
                                let is_target_ext = p.extension()
                                 .and_then(|ext| ext.to_str())
                                 .map_or(false, |ext| {
                                     ext.eq_ignore_ascii_case("STD")
                                 });

                                // 확장자가 맞다면, 파일명에 "__"가 포함되는지 확인
                                if is_target_ext {
                                    const EXCLUSIONS: &[&str] = &["_", "INIT", "복사본"];
                                    p.file_name()
                                     .and_then(|name| name.to_str())
                                      .map_or(false, |name_str| EXCLUSIONS.iter().all(|&exc| !name_str.contains(exc)))
                                } else {
                                    false
                                }
                            });
                        }
                        if should_rescan {
                            match get_base_std_files(&dir_to_watch) {
                                Ok(filtered_files) => {
                                    let payload = FileNamesPayload { data: filtered_files, };

                                    if let Err(e) = app_handle.emit("onUpdateBaseStdFiles", payload) {
                                        eprintln!("필터링된 파일 목록 이벤트 전송 오류: {:?}", e);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("디렉토리 파일 검사 오류: {:?}", e);
                                }
                            }
                        }
                    } else {
                        println!("파일 이벤트 채널이 닫혀서 감시 태스크 종료");
                        break;
                    }
                },
                // 2. 외부에서 oneshot 채널로 종료 신호가 도착함
                _ = &mut shutdown_rx => {
                    println!("종료 신호 수신: 파일 감시 태스크 종료");
                    break;
                }
            }
        }
    });

    Ok(handle)
}
