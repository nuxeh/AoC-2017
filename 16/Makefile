day=16
hook=../.git/hooks/prepare-commit-msg

dance: dance.rs
	rustc $@.rs

all: dance
	./dance

test: dance
	echo "s1,x3/4,pe/b" | ./dance

$(hook): .git_hook
	cat .git_hook > $@
	chmod +x $@

commit: $(hook)
	git commit
