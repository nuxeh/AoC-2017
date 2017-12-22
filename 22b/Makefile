day=22b
target=debug

debug_target=./virus/target/debug/virus
release_target=./virus/target/release/virus

src=./virus/src/main.rs

$(debug_target): $(src)
	cd virus && cargo build

$(release_target): $(src)
	cd virus && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

virus: $(target)

all: virus

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

test: $(release_target)
	cat test.txt | ./virus/target/release/virus

run: $(release_target)
	cat input.txt | ./virus/target/release/virus

clean:
	cd virus && cargo clean
