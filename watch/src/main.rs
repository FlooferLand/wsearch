use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use chrono::Utc;
use notify::{Result, Event, Watcher, RecursiveMode};

fn main() -> Result<()> {
	let (tx, rx) = mpsc::channel::<Result<Event>>();
	let mut watcher = notify::recommended_watcher(tx)?;
	watcher.watch(Path::new("./artworks"), RecursiveMode::Recursive)?;
	watcher.watch(Path::new("./website"), RecursiveMode::Recursive)?;

	// Watching the built out files
	let mut args = vec!["./build"];
	if !std::env::args().any(|a| a == "--open-browser") {
		args.push("--no-browser");
	}
	Command::new("live-server")
		.args(args)
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()?;

	// Watching the source files and rebuilding
	let mut time = Utc::now();
	let mut child = spawn();
	for res in rx {
		let Ok(res) = res else { continue };

		// Skip some paths
		let mut skip = false;
		for path in &res.paths {
			if path.extension().unwrap_or_default().to_string_lossy().ends_with("~") {
				skip = true; // Skip temporary files
				break;
			}
			if let Some(parent) = path.parent() {
				if parent.file_name().unwrap_or_default().to_string_lossy().starts_with("_") {
					skip = true;  // Skip unrelated auto-generated stuff
					break;
				}
			}
		}
		if skip {
			continue
		}

		// Retriggering build
		let now = Utc::now();
		if (now - time).as_seconds_f32() > 1.8 {
			let _ = child.kill();
			let _ = child.wait();
			child = spawn();
			time = now;
		}
	}
	Ok(())
}

fn spawn() -> Child {
	Command::new("cargo")
		.args(["run", "--package", "website", "--bin", "website", "--", "--served"])
		.stdout(Stdio::inherit())
		.stderr(Stdio::inherit())
		.spawn()
		.unwrap()
}
