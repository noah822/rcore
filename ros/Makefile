ENTRY_ADDR=0x80200000
OS_BIN_DIR=target/riscv64gc-unknown-none-elf/release

ros_uncheck: ros_bin
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=${OS_BIN_DIR}/ros.bin,addr=${ENTRY_ADDR} \

ros: ros_bin
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=${OS_BIN_DIR}/ros.bin,addr=${ENTRY_ADDR} \
		-s -S

ros_bin:
	cargo build --release
	rust-objcopy --strip-all ${OS_BIN_DIR}/ros -O binary ${OS_BIN_DIR}/ros.bin

gdb:
	riscv64-unknown-elf-gdb \
	    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
	    -ex 'set arch riscv:rv64' \
	    -ex 'target remote localhost:1234'