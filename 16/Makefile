day=16

dance: $@.rs
	rustc $@.rs

all: dance
	./dance


$(hook): .git_hook
	cat .git_hook > $@
	chmod +x $@

commit: $(hook)
	git commit
