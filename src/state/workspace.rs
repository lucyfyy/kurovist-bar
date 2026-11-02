use crate::platform::hyprland::models::Workspace;


pub fn visible_workspaces(
    workspaces: Vec<Workspace>,
    active: i32,
) -> Vec<i32> {

    let mut result: Vec<i32> = workspaces
        .into_iter()
        .filter(|ws| {
            ws.id == active || ws.windows > 0
        })
        .map(|ws| ws.id)
        .collect();


    result.sort();


    result
}