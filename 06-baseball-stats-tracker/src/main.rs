use baseball_stats_tracker::{Player, StatsTracker};

fn main() {
    println!("Hello baseball statistics tracker!");

    let mut tracker = StatsTracker::new();

    let mut trout = Player::new("Mike Trout".to_string(), "Los Angeles Angels".to_string(), "CF".to_string());

    trout.batting_stats.at_bats = 500;
    trout.batting_stats.hits = 165;
    trout.batting_stats.singles = 90;
    trout.batting_stats.doubles = 30;
    trout.batting_stats.triples = 5;
    trout.batting_stats.home_runs = 40;
    trout.batting_stats.runs_batted_in = 104;
    trout.batting_stats.walks = 85;
    trout.batting_stats.strikeouts = 120;

    let mut judge = Player::new("Aaron Judge".to_string(),"Yankees".to_string(),"RF".to_string());
    judge.batting_stats.at_bats = 550;
    judge.batting_stats.hits = 175;
    judge.batting_stats.singles = 80;
    judge.batting_stats.doubles = 32;
    judge.batting_stats.triples = 2;
    judge.batting_stats.home_runs = 62;
    judge.batting_stats.runs_batted_in = 131;
    judge.batting_stats.walks = 111;

    let mut betts = Player::new("Mookie Betts".to_string(),"Dodgers".to_string(), "RF".to_string());
    betts.batting_stats.at_bats = 520;
    betts.batting_stats.hits = 167;
    betts.batting_stats.singles = 88;
    betts.batting_stats.doubles = 35;
    betts.batting_stats.triples = 4;
    betts.batting_stats.home_runs = 42;
    betts.batting_stats.runs_batted_in = 82;
    betts.batting_stats.walks = 68;

    match tracker.add_player(trout) {
        Ok(_) => println!("✅ Added Mike Trout"),
        Err(e) => println!("❌ Error: {:?}", e),
    }
    
    match tracker.add_player(judge) {
        Ok(_) => println!("✅ Added Aaron Judge"),
        Err(e) => println!("❌ Error: {:?}", e),
    }
    
    match tracker.add_player(betts) {
        Ok(_) => println!("✅ Added Mookie Betts"),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    println!("\n🏁 Total players: {}", tracker.count());

    // Display leaderboards
    println!("\n🏆 LEADERBOARD BY OPS:");
    for (i, player) in tracker.leaderboard_by_ops().iter().enumerate() {
        println!("{}. {}", i + 1, player);
    }

    println!("\n💪 LEADERBOARD BY HOME RUNS:");
    for (i, player) in tracker.leaderboard_by_home_runs().iter().enumerate() {
        println!("{}. {}", i + 1, player);
    }

     // Test finding a player
    println!("\n🤔 Finding player...");
    match tracker.find_player("Aaron Judge") {
        Ok(player) => println!("🔍 Found: {}", player),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    // Test error handling - try to find non-existent player
    match tracker.find_player("Babe Ruth") {
        Ok(player) => println!("🔍 Found: {}", player),
        Err(e) => println!("❌ Error: {:?}", e),
    }

    println!("\n💾 Saving tracker to file...");
    match tracker.save_to_file("players.json") {
        Ok(_) => println!("✅ Successfully saved to players.json"),
        Err(e) => println!("❌ Failed to save: {}", e),
    }

    println!("\n📂 Loading tracker from file...");
    match StatsTracker::load_from_file("players.json") {
        Ok(loaded_tracker) => {
            println!("✅ Successfully loaded! Found {} players", loaded_tracker.count());
            println!("\n🏆 Loaded Leaderboard:");
            for (i, player) in loaded_tracker.leaderboard_by_ops().iter().enumerate() {
                println!("{}. {}", i + 1, player);
            }
        },
        Err(e) => println!("❌ Failed to load: {}", e),
    }

}
