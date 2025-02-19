use crate::format::CodeStr;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
    io,
    path::PathBuf,
    time::Duration,
};

// The program state
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct State {
    // Map from image ID to last use time expressed as a duration since the UNIX epoch
    pub images: HashMap<String, Duration>,
}

// Where the program state is persisted on disk
fn path() -> Option<PathBuf> {
    // [tag:state_path_has_parent]
    dirs::data_local_dir().map(|path| path.join("docuum/state.yml"))
}

// Return the state in which the program starts, if no state was loaded from disk.
pub fn initial() -> State {
    State {
        images: HashMap::new(),
    }
}

// Load the program state from disk.
pub fn load() -> io::Result<State> {
    // Check if we have a path.
    if let Some(path) = path() {
        // Log what we are trying to do in case an error occurs.
        debug!(
            "Attempting to load the state from {}\u{2026}",
            path.to_string_lossy().code_str(),
        );

        // Read the YAML from disk.
        let yaml = read_to_string(path)?;

        // Deserialize the YAML.
        serde_yaml::from_str(&yaml).map_err(|error| io::Error::new(io::ErrorKind::Other, error))
    } else {
        // Fail if we don't have a path.
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to locate data directory.",
        ))
    }
}

// Save the program state to disk.
pub fn save(state: &State) -> io::Result<()> {
    // Check if we have a path.
    if let Some(path) = path() {
        // Log what we are trying to do in case an error occurs.
        debug!(
            "Persisting the state to {}\u{2026}",
            path.to_string_lossy().code_str(),
        );

        // The `unwrap` is safe due to [ref:state_path_has_parent].
        let parent = path.parent().unwrap().to_owned();

        // The `unwrap` is safe because serialization should never fail.
        let payload = serde_yaml::to_string(state).unwrap();

        // Create the ancestor directories, if needed.
        create_dir_all(parent)?;

        // Write to the file.
        write(path, payload.as_bytes())?;
    } else {
        // Fail if we don't have a path.
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Unable to locate data directory.",
        ));
    }

    Ok(())
}
