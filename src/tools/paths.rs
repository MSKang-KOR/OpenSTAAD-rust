use anyhow::{Result, anyhow};
use std::path::{Path, PathBuf};
use winreg::{
    RegKey,
    enums::{HKEY_LOCAL_MACHINE, KEY_READ},
};

pub fn get_staad_exe_path() -> Result<PathBuf> {
    const UNINSTALL_KEYS: [&str; 2] = [
        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
    ];

    // HKEY_LOCAL_MACHINE 프리셋 키 가져오기
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // 각 Uninstall 경로를 순회하며 Staad.pro 관련 키를 찾습니다.
    for uninstall_path in &UNINSTALL_KEYS {
        match hklm.open_subkey_with_flags(uninstall_path, KEY_READ) {
            Ok(uninstall_key) => {
                // 서브키(각 설치된 프로그램의 GUID)들을 순회합니다.
                for subkey_name in uninstall_key.enum_keys().filter_map(|x| x.ok()) {
                    let subkey_path = format!(r"{}\{}", uninstall_path, subkey_name);

                    if let Ok(subkey) = hklm.open_subkey_with_flags(&subkey_path, KEY_READ) {
                        // 1. DisplayName을 확인하여 Staad.pro 관련 프로그램인지 판단
                        if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                            if display_name.to_lowercase().contains("staad.pro") {
                                // 2. InstallLocation 또는 InstallDir 값을 읽어 실행 파일 경로를 추측
                                if let Ok(install_location) =
                                    subkey.get_value::<String, _>("InstallLocation")
                                {
                                    let exe_path = PathBuf::from(format!(
                                        r"{}\STAAD\Bentley.Staad.exe",
                                        install_location.trim_end_matches('\\')
                                    ));
                                    if Path::exists(&exe_path) {
                                        if let Ok(metadata) = std::fs::metadata(&exe_path) {
                                            if metadata.is_file() {
                                                return Ok(exe_path);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // 해당 레지스트리 경로가 없으면 무시하고 다음 경로로 이동
                continue;
            }
        }
    }
    Err(anyhow!("Staad.pro 실행 파일 경로를 찾을 수 없습니다."))
}
