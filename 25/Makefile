day=25
target=debug

debug_target=./turing/target/debug/turing
release_target=./turing/target/release/turing

src=./turing/src/main.rs

$(debug_target): $(src)
	cd turing && cargo build

$(release_target): $(src)
	cd turing && cargo build --release

debug: $(debug_target)
	$(debug_target)

release: $(release_target)
	$(release_target)

turing: $(target)

all: turing

vi:
	vi $(src)

.git_template:
	echo "$(day): " > .git_template

commit: .git_template
	git commit -st .git_template

add:
	git add $(src)
	git diff --cached

amend:
	git commit --amend --no-edit

autocommit: add commit

autocommit_out: .git_template $(debug_target)
	cat .git_template > /tmp/commit_template
	echo "" >> /tmp/commit_template
	echo "    \$$ $(debug_target)" >> /tmp/commit_template
	$(debug_target) | sed 's/^/    /' >> /tmp/commit_template
	git commit -st /tmp/commit_template

test: $(debug_target)
	$(debug_target)
test_release: $(release_target)
	cat test.txt | $(release_target)
run: $(debug_target)
	cat input.txt | $(debug_target)
run_release: $(release_target)
	cat input.txt | $(release_target)
