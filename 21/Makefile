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

picture: $(target)

all: picture

vi:
	vi $(src)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -st .git_template

add:
	git add \(src)
