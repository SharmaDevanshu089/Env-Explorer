# Env Explorer Backend API Documentation

This document describes all modules and functions in the `src-tauri/src` directory of **Env Explorer**, detailing their inputs, outputs, types, behaviors, and frontend integration examples.

---

## Directory Overview (`src-tauri/src`)

The Tauri backend is organized into the following files:
1. [main.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/main.rs) - Application entry point and logging configuration.
2. [lib.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/lib.rs) - Tauri builder and command registration.
3. [crawl.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/crawl.rs) - Scanning filesystem for `.env` files.
4. [reader.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/reader.rs) - Reading parsed project configuration and environment variables.
5. [syshandler.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/syshandler.rs) - Injecting environment variables into the active process memory.
6. [terminal.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/terminal.rs) - Spawning external shell terminals loaded with specific environments.
7. [envirment.rs](file:///c:/Users/sharm/Env%20Explorer/src-tauri/src/envirment.rs) - Writing persistent global environment variables into the Windows Registry.

---

## 1. Application Entry Point (`main.rs`)

`main.rs` configures runtime logging and boots the application.

### `main` (Helper Function)
- **Scope**: Core Binary Entry Point
- **Inputs**: None
- **Outputs**: None
- **Simple Description**:
  Initializes logging by creating two log targets under `simplelog`:
  1. A file logger (`WriteLogger`) that writes all debug and info events to `env_explorer.log` in the running directory.
  2. A terminal logger (`SimpleLogger`) that prints log messages directly to the debug terminal console.
  After logging is active, it calls the library runner `env_explorer_lib::run()` to launch Tauri.

---

## 2. Tauri Configuration (`lib.rs`)

`lib.rs` initializes and boots the Tauri environment, specifying plugin connections and registering which functions are invokable from the frontend.

### `run` (Helper Function)
- **Scope**: Library Runner Entry Point
- **Inputs**: None
- **Outputs**: None
- **Simple Description**:
  Constructs a standard `tauri::Builder` instance, initializes the Tauri Opener plugin (`tauri_plugin_opener`), registers backend invoke handlers (allowing frontend code to call them), and boots the event loop.

---

## 3. Directory Crawl (`crawl.rs`)

`crawl.rs` handles filesystem scanning to discover `.env` files.

### `intiate_crawl` (Tauri Command)
- **Typo Note**: Function name contains a typo: `intiate_crawl` (missing the second 'i' in initiate).
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `app_handle: tauri::AppHandle` (automatically injected by Tauri)
- **Outputs**: None (`()`)
- **Simple Description**:
  Finds the user's home directory and performs a deep scan for `.env` files. It excludes common large directories (like `node_modules`, `target`, `AppData`, etc.) and hidden directories. Once found, it saves the list of projects (name and file path) as a formatted JSON array to `env_config.json` in the application's local data directory.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  // Starts background crawl and updates env_config.json
  await invoke("intiate_crawl");
  ```

### `scan` (Helper Function)
- **Scope**: Internal Rust function
- **Inputs**:
  - `next_directory_path: std::path::PathBuf` (current folder path to scan)
  - `env_files: &mut Vec<ProjectEnv>` (mutable reference to list where found projects are stored)
- **Outputs**: None (`()`)
- **Simple Description**:
  Recursively crawls folders. If a subdirectory matches a known folder to ignore, it skips it. If it encounters a file named `.env`, it parses the parent folder's name as the project name and pushes the project configuration to `env_files`.

---

## 4. Config & Env Reader (`reader.rs`)

`reader.rs` handles loading cached projects and reading variable key-values.

### `read_env_config` (Tauri Command)
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `app_handle: tauri::AppHandle` (automatically injected by Tauri)
- **Outputs**:
  - `Result<Vec<ProjectEnv>, String>`
  - Where `ProjectEnv` represents:
    ```rust
    pub struct ProjectEnv {
        pub name: String,
        pub path: String,
    }
    ```
- **Simple Description**:
  Reads the cached project list from `env_config.json` in the app local data folder. If the file doesn't exist yet, it returns an empty list. Otherwise, it parses and returns the list of all found `.env` files.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  try {
    const projects = await invoke("read_env_config");
    // Returns: [{ name: "my-app", path: "/users/name/my-app/.env" }]
    console.log(projects);
  } catch (error) {
    console.error("Failed to read config:", error);
  }
  ```

### `get_current_env_vars` (Tauri Command)
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `path: String` (absolute file path to a `.env` file)
- **Outputs**:
  - `HashMap<String, String>` (key-value mapping of environment variables)
- **Simple Description**:
  Opens the `.env` file at the specified file path, parses it line-by-line (filtering out comments and empty lines), and returns a dictionary of the environment variables.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  const path = "/users/name/projects/my-app/.env";
  const envVars = await invoke("get_current_env_vars", { path });
  // Returns: { "DATABASE_URL": "postgresql://...", "PORT": "3000" }
  console.log(envVars);
  ```

### `count_env_vars` (Tauri Command)
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `path: String` (absolute file path to a `.env` file)
- **Outputs**:
  - `usize` (the total number of keys)
- **Simple Description**:
  Parses the `.env` file at the specified path and counts how many valid environment variables it contains, returning the total as an integer.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  const path = "/users/name/projects/my-app/.env";
  const count = await invoke("count_env_vars", { path });
  console.log(`Total variables: ${count}`);
  ```

---

## 5. System Environment Handler (`syshandler.rs`)

`syshandler.rs` handles injecting variable values into running memory contexts.

### `load_env_to_system_process` (Tauri Command)
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `path: String` (absolute file path to a `.env` file)
- **Outputs**:
  - `Result<bool, String>`
- **Simple Description**:
  Loads the environment variables from the given `.env` file directly into the **running environment of the Tauri application process** itself. Any child processes spawned by Tauri after calling this will inherit these environment variables.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  try {
    const success = await invoke("load_env_to_system_process", { path: "/path/to/.env" });
    if (success) console.log("Loaded variables to Tauri process!");
  } catch (err) {
    console.error("Failed to load:", err);
  }
  ```

---

## 6. Terminal Launcher (`terminal.rs`)

`terminal.rs` spawns shell environments configured with project variables.

### `launch_terminal_with_env` (Tauri Command)
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `path: String` (absolute file path to a `.env` file)
- **Outputs**:
  - `Result<bool, String>`
- **Simple Description**:
  Launches a new, standalone Windows Command Prompt (`cmd.exe`) window, sets its working directory to the parent folder of the `.env` file, and pre-loads all environment variables parsed from that file into the terminal context.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  try {
    await invoke("launch_terminal_with_env", { path: "/path/to/.env" });
    console.log("Terminal window launched successfully!");
  } catch (err) {
    console.error("Failed to launch terminal:", err);
  }
  ```

---

## 7. OS Environment Keys Registry (`envirment.rs`)

`envirment.rs` manages persistent OS environment registry keys.

### `add_user_env_var` (Tauri Command)
- **Scope**: Tauri Command (Invokable from frontend)
- **Inputs**:
  - `key: String` (environment variable name)
  - `value: String` (environment variable value)
- **Outputs**:
  - `Result<bool, String>`
- **Simple Description**:
  Writes the given environment variable directly to the **Windows User Registry** under the path `HKCU\Environment` so that it persists globally for the current user. To make sure all other running programs pick up the change immediately without requiring a system reboot, it runs a PowerShell helper to broadcast a `WM_SETTINGCHANGE` message to the OS.
- **Frontend Invoke Example**:
  ```javascript
  import { invoke } from "@tauri-apps/api/core";

  try {
    const success = await invoke("add_user_env_var", { 
      key: "MY_GLOBAL_KEY", 
      value: "super-secret-value" 
    });
    if (success) console.log("Registry key written and broadcasted!");
  } catch (err) {
    console.error("Failed to write to registry:", err);
  }
  ```
