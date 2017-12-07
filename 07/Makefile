all:
	rustc process_tower.rs

run: all
	./process_tower

part_a:
	./filter.awk input.txt > input_f.txt
	./process.awk input_f.txt
