day=17
hook=../.git/hooks/prepare-commit-msg

spinlock: spinlock.rs
	rustc $@.rs

all: spinlock
	./spinlock


$(hook): .git_hook
	cat .git_hook > $@
	chmod +x $@

commit: $(hook)
	git commit
