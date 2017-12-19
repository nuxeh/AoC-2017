day=19
target=debug

debug_target=./path/target/debug/path
release_target=./path/target/release/path

src=./path/src/main.rs

$(debug_target): $(src)
	cd path && cargo build

$(release_target): $(src)
	cd path && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

path: $(target)

all: path

vi:
	vi $(src)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -st .git_template
