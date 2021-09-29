use bevy::{prelude::*,};

#[derive(Default)]
pub struct AssetsLoading {
    pub asset_handles: Vec<HandleUntyped>
}
pub fn check_assets_ready(
    mut state: ResMut<State<crate::AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    println!("Loading...");
    use bevy::asset::LoadState;

    let mut ready = true;

    for handle in loading.asset_handles.iter() {
        match server.get_load_state(handle) {
            LoadState::Failed => {
                // one of our assets had an error
            }
            LoadState::Loaded => {
            }
            _ => {
                ready = false;
            }
        }
    }

    if ready {
        state.set(crate::AppState::InGame).unwrap();
    }
}
