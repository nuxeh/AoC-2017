day=24
target=debug

debug_target=./bridge/target/debug/bridge
release_target=./bridge/target/release/bridge

src=./bridge/src/main.rs

$(debug_target): $(src)
	cd bridge && cargo build

$(release_target): $(src)
	cd bridge && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

bridge: $(target)

all: bridge

vi:
	vi $(src)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -st .git_template

add:
	git add $(src)
	git diff --cached

autocommit: add commit

test: $(debug_target)
	cat test.txt | $(debug_target)
test_release: $(release_target)
	cat test.txt | $(release_target)
run: $(debug_target)
	cat input.txt | $(debug_target)
run_release: $(release_target)
	cat input.txt | $(release_target)
