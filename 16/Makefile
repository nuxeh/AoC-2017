day=16
hook=../create_prapare_commit_hook.sh

dance: $@.rs
	rustc $@.rs

all: dance
	./dance


$(hook): .git_hook
	cat .git_hook > $@
	chmod +x $@

commit: $(hook)
	git commit
