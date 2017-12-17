day=17
debug_target=./spin_lock/target/debug/spin_lock
release_target=./spin_lock/target/release/spin_lock


debug_target: ./spin_lock/src/main.rs
	cd spin_lock && cargo +nightly build

release_target: ./spin_lock/src/main.rs
	cd spin_lock && cargo +nightly build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	time $(release_target)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -t .git_template
