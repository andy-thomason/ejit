
listings:
	# x86_64-linux-gnu-as asm/x86_64.s -a -o target/a.out; objdump -d target/a.out> asm/vector_aarch64.lst
	aarch64-linux-gnu-as asm/aarch64.s -o target/a.out; objdump -d target/a.out> asm/aarch64.lst

vector: asm/vector.c
	mkdir -p src/aarch64/
	aarch64-linux-gnu-gcc asm/vector.c -O -o target/a.out; objdump -d target/a.out> asm/vector_aarch64.lst
	cargo run --bin make_vector
