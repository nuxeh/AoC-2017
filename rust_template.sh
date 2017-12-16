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

cat > .git_hook << EOF
#!/bin/bash
#
# Git commit prepare hook

# TODO: don't replace if already present (amend)
D=$2
sed -i "1i\$D: " \$1
EOF

echo '.*.sw*' > .gitignore
echo "$1" >> .gitignore

cat > Makefile << EOF
day=$2
hook=../.git/hooks/prepare-commit-msg

$1: $1.rs
	rustc \$@.rs

all: $1
	./$1


\$(hook): .git_hook
	cat .git_hook > \$@
	chmod +x \$@

commit: \$(hook)
	git commit
EOF

git add Makefile $1.rs .gitignore .git_hook
git ct -m "$2: add rust template"
