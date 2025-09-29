if [ "$1" = "release" ]; then
	cargo build -r 
	mkdir -p bin/release
	cp target/release/get_size bin/release
	echo "The process is complete, you can run ./bin/release/get_size"
elif [ "$1" = "debug" ]; then
	cargo build
	mkdir -p bin/debug
	cp target/debug/get_size bin/debug
	echo "The process is complete, you can run ./bin/debug/get_size"
else 
	echo "Error: Please specify release or debug arguments."
fi
