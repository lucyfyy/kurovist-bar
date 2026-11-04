use std::collections::BTreeSet;
use crate::platform::hyprland::models::Workspace;

pub fn calculate_visible_workspaces(
    workspaces: Vec<Workspace>, 
    active_id: i32
) -> Vec<i32> {
    // Gunakan BTreeSet agar tidak ada ID duplikat dan otomatis terurut
    let mut visible_set: BTreeSet<i32> = BTreeSet::new();

    // 1. Selalu tampilkan workspace default 1 sampai 5
    for id in 1..=5 {
        visible_set.insert(id);
    }

    // 2. Tambahkan workspace yang punya window/aplikasi aktif
    for ws in workspaces {
        // Asumsi struct Workspace kamu punya field id dan windows (jumlah window)
        // Jika ws.id > 0 dan memiliki setidaknya 1 window, tampilkan!
        if ws.id > 0 && ws.windows > 0 {
            visible_set.insert(ws.id);
        }
    }

    // 3. Tambahkan workspace tempat kursor/fokus kita berada sekarang (active workspace)
    if active_id > 0 {
        visible_set.insert(active_id);
    }

    // Convert ke Vector yang sudah rapi terurut
    visible_set.into_iter().collect()
}