use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("storage-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::SeqCst)
            ));
        fs::create_dir_all(&root).unwrap();
        Self(root)
    }
    fn paths(&self, name: &str) -> StoragePaths {
        StoragePaths {
            root: self.0.join(name),
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn install_locations_cover_bundles_and_appimage() {
    let f = Fixture::new();
    assert_eq!(
        installation_root(&f.0.join("pilot/frpcpilot"), None, false).unwrap(),
        f.0.join("pilot")
    );
    assert_eq!(
        installation_root(
            &f.0.join(".mount_Pilot/usr/bin/frpcpilot"),
            Some(&f.0.join("apps/FrpcPilot.AppImage")),
            false
        )
        .unwrap(),
        f.0.join("apps")
    );
    assert_eq!(
        installation_root(
            &f.0.join("Applications/FrpcPilot.app/Contents/MacOS/frpcpilot"),
            None,
            true
        )
        .unwrap(),
        f.0.join("Applications/FrpcPilot-data")
    );
    assert!(installation_root(Path::new("relative/frpcpilot"), None, false).is_err());
}

#[test]
fn fresh_install_and_exclusive_storage_lock() {
    tauri::async_runtime::block_on(async {
        let f = Fixture::new();
        let paths = f.paths("new");
        let (pool, lock) = initialize(&paths).await.unwrap();
        assert!(paths.db().is_file());
        assert!(paths.run().is_dir());
        assert!(paths.versions().is_dir());
        assert!(initialize(&paths).await.is_err());
        pool.close().await;
        drop(lock);
        let (pool, _lock) = initialize(&paths).await.unwrap();
        pool.close().await;
    });
}
