use std::fs;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub icon: String,
}

pub fn load_applications() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let search_paths = vec!["/usr/share/applications"];

    for path_str in search_paths {
        let path = Path::new(path_str);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if file_path.extension().and_then(|s| s.to_str()) == Some("desktop") {
                    if let Some(app) = parse_desktop_file(&file_path) {
                        apps.push(app);
                    }
                }
            }
        }
    }
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn parse_desktop_file(path: &Path) -> Option<DesktopApp> {
    let content = fs::read_to_string(path).ok()?;
    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut no_display = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("NoDisplay=true") {
            no_display = true;
        }
        if name.is_none() && line.starts_with("Name=") {
            name = Some(line.strip_prefix("Name=")?.to_string());
        }
        if exec.is_none() && line.starts_with("Exec=") {
            let raw_exec = line.strip_prefix("Exec=")?;
            let clean_exec = raw_exec.split('%').next().unwrap_or(raw_exec).trim().to_string();
            exec = Some(clean_exec);
        }
        if icon.is_none() && line.starts_with("Icon=") {
            icon = Some(line.strip_prefix("Icon=")?.to_string());
        }
    }

    if no_display { return None; }

    Some(DesktopApp {
        name: name?,
        exec: exec?,
        icon: icon.unwrap_or_else(|| "application-x-executable".to_string()),
    })
}