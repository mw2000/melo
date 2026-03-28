use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver};

use color_eyre::Result;
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

/// Keeps the underlying watcher alive; dropping this struct stops file watching.
pub struct FileWatcher {
    _watcher: RecommendedWatcher,
}

impl FileWatcher {
    pub fn new(path: PathBuf) -> Result<(Self, Receiver<()>)> {
        let (tx, rx) = mpsc::sync_channel(1);

        let mut watcher = RecommendedWatcher::new(
            move |result: notify::Result<Event>| {
                let Ok(event) = result else {
                    return;
                };

                let changed = matches!(
                    event.kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                );

                if changed {
                    let _ = tx.try_send(());
                }
            },
            Config::default(),
        )?;

        watcher.watch(&path, RecursiveMode::NonRecursive)?;

        Ok((Self { _watcher: watcher }, rx))
    }
}
