use tauri::{plugin::{Builder, TauriPlugin}, Runtime};
use tauri::State;

use crate::device_manager::{Device, DeviceManager, Error};

#[tauri::command]
async fn list(manager: State<'_, DeviceManager>) -> Result<Vec<Device>, Error> {
  return manager.list().await;
}

#[tauri::command]
async fn set_default(manager: State<'_, DeviceManager>, name: String) -> Result<Option<Device>, Error> {
  return manager.set_default(&name).await;
}

#[tauri::command]
async fn remove(manager: State<'_, DeviceManager>, name: String) -> Result<(), Error> {
  return manager.remove(&name).await;
}

/// Initializes the plugin.
pub fn plugin<R: Runtime>(name: &'static str) -> TauriPlugin<R> {
  Builder::new(name)
    .invoke_handler(tauri::generate_handler![list, set_default, remove])
    .build()
}
