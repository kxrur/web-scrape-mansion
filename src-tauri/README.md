# Tauri Bundle - Build

`NO_STRIP=true npm run tauri build -- --verbose`

# Tauri Store

[store](https://v2.tauri.app/develop/state-management/)

# Typesafe Interface with TS

[tauri_specta](https://docs.rs/tauri-specta/2.0.0-rc.20/tauri_specta/index.html)

- project has to run for functions to export to the frontend file
  (`../src/bindings`)
- additional `#[specta::specta]` annotation on exported `functions`
- additional `#[derive(Serialize, Deserialize, Type)]` annotation on exported types (i.e. exported function's return / input types)
