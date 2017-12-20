day=20
target=debug

debug_target=./gpu/target/debug/gpu
release_target=./gpu/target/release/gpu

src=./gpu/src/main.rs

$(debug_target): $(src)
	cd gpu && cargo build

$(release_target): $(src)
	cd gpu && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

gpu: $(target)

all: gpu

vi:
	vi $(src)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -st .git_template
