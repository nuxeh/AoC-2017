#!/bin/bash
#
# Create rust template

if [[ $# -ne 2 ]]; then
	printf "usage:\n    $0 [name] [day-number]\n"
	exit 1
fi

cat > "$1.rs" << EOF
fn main () {

}
EOF

cat > .git_template << EOF
$2: 
EOF

echo '.*.sw*' > .gitignore
echo "$1" >> .gitignore

cat > Makefile << EOF
day=$2

$1: $1.rs
	rustc \$@.rs

all: $1
	./$1

commit: \$(hook)
	git commit -t .git_template
EOF

git add Makefile $1.rs .gitignore .git_hook
git ct -m "$2: add rust template"
