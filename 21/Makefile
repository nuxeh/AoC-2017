day=21
target=debug

debug_target=./picture/target/debug/picture
release_target=./picture/target/release/picture

src=./picture/src/main.rs

$(debug_target): $(src)
	cd picture && cargo build

$(release_target): $(src)
	cd picture && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

enable-backtrace:
	export RUST_BACKTRACE=1

picture: $(target)

all: picture enable-backtrace

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

run: $(debug_target)
	cat input.txt | picture/target/debug/picture

run_release: $(release_target)
	cat input.txt | picture/target/release/picture
