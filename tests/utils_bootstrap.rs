use std::{cell::RefCell, env::var, path::PathBuf, process::Command, thread, time::Duration};

const PROJECT_ROOT: &'static str = env!("CARGO_MANIFEST_DIR");
const BOOTSTRAPPED: RefCell<bool> = RefCell::new(false);

pub fn bootstrap() {
    if *BOOTSTRAPPED.get_mut() {
        // already bootstrapped
        return;
    }
    match var("CDRS_LOCAL_TEST_DB") {
        Ok(db) => match db.as_str() {
            "cassandra" => start_cassandra(),
            "scylla" => start_scylla(),
            unsupported_db => panic!(format!("local db is not supported: {}", unsupported_db)),
        },
        Err(_) => {}
    }
}

fn start_cassandra() {
    let start_script_path: PathBuf = [PROJECT_ROOT, "tests/scripts/start_cassandra.sh"]
        .iter()
        .collect();

    Command::new("sh")
        .arg(
            start_script_path
                .to_str()
                .expect("start Cassandra script path"),
        )
        .output()
        .expect("start Cassandra script run");

    *BOOTSTRAPPED.get_mut() = true;
    thread::sleep(Duration::from_secs(10));
}

fn start_scylla() {
    let start_script_path: PathBuf = [PROJECT_ROOT, "tests/scripts/start_scylla.sh"]
        .iter()
        .collect();

    Command::new("sh")
        .arg(
            start_script_path
                .to_str()
                .expect("start Scylla script path"),
        )
        .output()
        .expect("start Scylla script run");

    *BOOTSTRAPPED.get_mut() = true;
    thread::sleep(Duration::from_secs(10));
}
