day=17

spinlock: spinlock.rs
	rustc $@.rs

all: spinlock
	./spinlock

.git_template:
	echo " " > .git_template

commit: .git_template
	git commit -t .git_template
