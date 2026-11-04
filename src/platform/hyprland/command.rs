use std::process::Command;

pub fn dispatch(lua_cmd: &str) {
    let _ = Command::new("hyprctl")
        .args(["dispatch", lua_cmd])
        .spawn();
}

pub fn workspace(id: i32) {
    // Memformat perintah dispatch sesuai sintaks Lua API Hyprland: hl.dsp.focus({ workspace = "id" })
    let lua_cmd = format!("hl.dsp.focus({{ workspace = \"{}\" }})", id);
    dispatch(&lua_cmd);
}