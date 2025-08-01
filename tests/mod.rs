mod snapshot_runner;
use mcplease::traits::AsToolsList;
use semantic_code_edit_mcp::tools::Tools;
use snapshot_runner::SnapshotRunner;
use std::env;

#[test]
fn run_snapshot_tests() {
    let update_mode = env::var("UPDATE_SNAPSHOTS").is_ok() || env::var("UPDATE_SNAPSHOT").is_ok();
    let test_filter = env::var("TEST_FILTER").ok();

    let mut runner =
        SnapshotRunner::new(update_mode, test_filter).expect("Failed to create snapshot runner");

    let results = runner
        .run_all_tests()
        .expect("Failed to run snapshot tests");

    runner.print_summary(&results);

    // In verify mode, fail if any tests failed
    if !update_mode {
        let failed_count = results
            .iter()
            .filter(|r| !r.response_matches || !r.output_matches)
            .count();
        if failed_count > 0 {
            panic!("❌ {failed_count} snapshot test(s) failed");
        }
    }
}

#[test]
fn tools_doesnt_panic() {
    Tools::tools_list();
}
