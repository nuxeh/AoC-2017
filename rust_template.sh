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

fn main () {
	part1();
	part2();
}

fn part1() {

}

fn part2() {

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

autocommit: add commit
EOF

git add Makefile "$1" .gitignore
git ct -m "$d: add rust template"
