#!/bin/bash
#
# Create rust template

if [[ $# -ne 2 ]]; then
	printf "usage:\n    $0 [name] [day-number]\n"
	exit 1
fi

echo "Creating empty rust project \"$1\" for day \"$2\""

cargo new "$1" --bin

cat > "./$1/src/main.rs" << EOF
/* Advent of code */
/* day $1 */

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
echo "$1" >> .gitignore
echo ".git_template" >> .gitignore

cat > Makefile << EOF
day=$2

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

$1: release

all: $1
	./$1

.git_template:
	echo "\$(day): " > .git_template

commit: .git_template
	git commit -t .git_template
EOF

git add Makefile "$1" .gitignore
git ct -m "$2: add rust template"
