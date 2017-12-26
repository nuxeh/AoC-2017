#!/bin/bash
#
# Create rust template

d=$(basename $PWD)

if [[ $# -ne 1 ]]; then
	printf "usage:\n    $0 [name]\n"
	exit 1
fi

echo "Creating empty rust project \"$1\" for day $d"

cargo new "$1" --bin

cat > "./$1/src/main.rs" << EOF
/* Advent of code */
/* day $d */

use std::io;
use std::io::BufRead;

fn main () {
	read_stdin();

	part1();
	part2();
}

fn part1() {

}

fn part2() {

}

fn read_stdin() {
	let std = io::stdin();

	for l in std.lock().lines() {
		let l = l.unwrap();
		println!("{}", l);

	}
}
EOF

echo '.*.sw*' > .gitignore
echo ".git_template" >> .gitignore

cat > Makefile << EOF
day=$d
target=debug

debug_target=./$1/target/debug/$1
release_target=./$1/target/release/$1

src=./$1/src/main.rs

\$(debug_target): \$(src)
	cd $1 && cargo build

\$(release_target): \$(src)
	cd $1 && cargo build --release

debug: \$(debug_target)
	\$(debug_target)

release: \$(release_target)
	\$(release_target)

$1: \$(target)

all: $1

vi:
	vi \$(src)

.git_template:
	echo "\$(day): " > .git_template

commit: .git_template
	git commit -st .git_template

add:
	git add \$(src)
	git diff --cached

amend:
	git commit --amend --no-edit

autocommit: add commit

autocommit_out: .git_template \$(debug_target)
	cat .git_template > /tmp/commit_template
	echo "" >> /tmp/commit_template
	echo "    \\$\$ \$(debug_target)" >> /tmp/commit_template
	\$(debug_target) | sed 's/^/    /' >> /tmp/commit_template
	git commit -st /tmp/commit_template

test: \$(debug_target)
	cat test.txt | \$(debug_target)
test_release: \$(release_target)
	cat test.txt | \$(release_target)
run: \$(debug_target)
	cat input.txt | \$(debug_target)
run_release: \$(release_target)
	cat input.txt | \$(release_target)
EOF

git add Makefile "$1" .gitignore
git ct -m "$d: add rust template"

# TODO: Automatically commit output
