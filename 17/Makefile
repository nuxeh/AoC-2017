day=17

spinlock: spinlock.rs
	rustc $@.rs

all: spinlock
	./spinlock

commit: $(hook)
	git commit -t .git_template
