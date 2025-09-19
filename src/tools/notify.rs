use notify::{Event, EventKind, RecursiveMode, Result, Watcher};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};

/// 백그라운드에서 파일 감시를 시작하는 함수
/// JoinHandle을 반환하여 필요시 스레드를 제어할 수 있음
pub fn watch_file_background(path: &PathBuf, app: AppHandle) -> thread::JoinHandle<()> {
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
                        &app,
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
                // println!("{:#?}", new_line);
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
