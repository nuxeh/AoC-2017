day=18
target=debug

debug_target=./duet/target/debug/duet
release_target=./duet/target/release/duet

src=./duet/src/main.rs

$(debug_target): $(src)
	cd duet && cargo build

$(release_target): $(src)
	cd duet && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

duet: $(target)

all: duet

vi:
	vi $(src)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -st .git_template
