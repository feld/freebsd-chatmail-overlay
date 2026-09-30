--- src/main.rs.orig	2026-09-30 17:23:06 UTC
+++ src/main.rs
@@ -1,5 +1,6 @@ use std::net::IpAddr;
 use std::collections::BTreeSet;
 use std::net::IpAddr;
+use std::fs;
 use std::path::Path;
 use std::sync::Arc;
 use std::time::Duration;
@@ -44,6 +45,9 @@ async fn socket_loop(path: &Path, shared_secret: &str)
 /// Listens on the Unix socket,
 /// returning valid credentials to any connecting client.
 async fn socket_loop(path: &Path, shared_secret: &str) -> Result<()> {
+    if path.exists() {
+        fs::remove_file(path).context("Failed to remove stale Unix socket")?;
+    }
     let listener = UnixListener::bind(path).context("Failed to bind Unix socket")?;
     loop {
         match listener.accept().await {
