// Integration tests for baseball_stats_tracker
// These tests interact with the real file system and test the library as a whole

use baseball_stats_tracker::{BattingStats, Player, StatsTracker};
use tempfile::NamedTempFile;

fn create_test_player(name: &str, team: &str) -> Player {
    let mut player = Player::new(name.to_string(), team.to_string(), "CF".to_string());
    player.batting_stats = BattingStats {
        at_bats: 100,
        hits: 30,
        singles: 20,
        doubles: 5,
        triples: 2,
        home_runs: 3,
        runs_batted_in: 25,
        walks: 15,
        strikeouts: 20,
    };
    player
}

fn create_test_tracker() -> StatsTracker {
    let mut tracker = StatsTracker::new();
    tracker.add_player(create_test_player("Mike Trout", "Angels")).unwrap();
    tracker.add_player(create_test_player("Aaron Judge", "Yankees")).unwrap();
    tracker
}

#[test]
fn test_save_and_load_roundtrip() {
    // Create a temporary file that will be cleaned up automatically
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    // Create and save a tracker
    let original_tracker = create_test_tracker();
    let save_result = original_tracker.save_to_file(file_path);
    assert!(save_result.is_ok(), "Should successfully save to file");

    // Load the tracker back
    let load_result = StatsTracker::load_from_file(file_path);
    assert!(load_result.is_ok(), "Should successfully load from file");

    let loaded_tracker = load_result.unwrap();

    // Verify the data is the same
    assert_eq!(loaded_tracker.count(), 2, "Should have 2 players");

    let mike = loaded_tracker.find_player("Mike Trout");
    assert!(mike.is_ok(), "Should find Mike Trout");
    assert_eq!(mike.unwrap().team, "Angels");

    let aaron = loaded_tracker.find_player("Aaron Judge");
    assert!(aaron.is_ok(), "Should find Aaron Judge");
    assert_eq!(aaron.unwrap().team, "Yankees");
}

#[test]
fn test_load_nonexistent_file() {
    let result = StatsTracker::load_from_file("/nonexistent/path/that/does/not/exist.json");
    assert!(result.is_err(), "Should fail to load nonexistent file");

    // Check that it's an IoError
    match result {
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(error_message.contains("I/O Error"), "Error should be an I/O Error");
        },
        Ok(_) => panic!("Should have returned an error"),
    }
}

#[test]
fn test_load_invalid_json() {
    // Create a temporary file with invalid JSON
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    // Write invalid JSON to the file
    std::fs::write(file_path, "This is not valid JSON at all!").unwrap();

    // Try to load it
    let result = StatsTracker::load_from_file(file_path);
    assert!(result.is_err(), "Should fail to load invalid JSON");

    // Check that it's an IoError (from serde_json::Error conversion)
    match result {
        Err(e) => {
            let error_message = format!("{}", e);
            assert!(error_message.contains("I/O Error"), "Error should be an I/O Error");
        },
        Ok(_) => panic!("Should have returned an error"),
    }
}

#[test]
fn test_save_creates_pretty_json() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    let tracker = create_test_tracker();
    tracker.save_to_file(file_path).unwrap();

    // Read the file contents
    let contents = std::fs::read_to_string(file_path).unwrap();

    // Verify it's pretty-printed (has newlines and indentation)
    assert!(contents.contains('\n'), "Should be pretty-printed with newlines");
    assert!(contents.contains("  "), "Should be pretty-printed with indentation");
    assert!(contents.contains("\"players\""), "Should contain players field");
    assert!(contents.contains("Mike Trout"), "Should contain Mike Trout");
    assert!(contents.contains("Aaron Judge"), "Should contain Aaron Judge");
}

#[test]
fn test_save_and_load_empty_tracker() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    // Save an empty tracker
    let empty_tracker = StatsTracker::new();
    empty_tracker.save_to_file(file_path).unwrap();

    // Load it back
    let loaded_tracker = StatsTracker::load_from_file(file_path).unwrap();
    assert_eq!(loaded_tracker.count(), 0, "Should have 0 players");
}

#[test]
fn test_save_and_load_preserves_stats() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_str().unwrap();

    // Create a tracker with specific stats
    let mut tracker = StatsTracker::new();
    let mut player = Player::new("Test Player".to_string(), "Test Team".to_string(), "SS".to_string());
    player.batting_stats = BattingStats {
        at_bats: 450,
        hits: 135,
        singles: 80,
        doubles: 30,
        triples: 5,
        home_runs: 20,
        runs_batted_in: 85,
        walks: 60,
        strikeouts: 100,
    };
    tracker.add_player(player).unwrap();

    // Save and load
    tracker.save_to_file(file_path).unwrap();
    let loaded_tracker = StatsTracker::load_from_file(file_path).unwrap();

    // Verify all stats are preserved
    let loaded_player = loaded_tracker.find_player("Test Player").unwrap();
    assert_eq!(loaded_player.batting_stats.at_bats, 450);
    assert_eq!(loaded_player.batting_stats.hits, 135);
    assert_eq!(loaded_player.batting_stats.singles, 80);
    assert_eq!(loaded_player.batting_stats.doubles, 30);
    assert_eq!(loaded_player.batting_stats.triples, 5);
    assert_eq!(loaded_player.batting_stats.home_runs, 20);
    assert_eq!(loaded_player.batting_stats.runs_batted_in, 85);
    assert_eq!(loaded_player.batting_stats.walks, 60);
    assert_eq!(loaded_player.batting_stats.strikeouts, 100);

    // Verify calculated stats are correct
    assert_eq!(loaded_player.batting_stats.batting_average(), 0.3);
}
