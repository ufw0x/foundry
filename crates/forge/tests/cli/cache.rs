//! Tests for various cache command.

forgetest!(can_list_cache, |_prj, cmd| {
    cmd.args(["cache", "ls"]);
    cmd.assert_success();
});

forgetest!(can_list_cache_all, |_prj, cmd| {
    cmd.args(["cache", "ls", "all"]);
    cmd.assert_success();
});

forgetest!(can_list_specific_chain, |_prj, cmd| {
    cmd.args(["cache", "ls", "mainnet"]);
    cmd.assert_success();
});

forgetest_init!(can_test_no_cache, |prj, cmd| {
    let _ = std::fs::remove_dir_all(prj.cache_path());

    cmd.args(["test", "--no-cache"]).assert_success();
    assert!(!prj.cache_path().exists(), "cache file should not exist");

    cmd.forge_fuse().args(["test"]).assert_success();
    assert!(prj.cache_path().exists(), "cache file should exist");
});
