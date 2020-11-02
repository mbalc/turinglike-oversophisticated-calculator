all: interpreter translate

interpreter: target/debug/interpreter
	cp $^ ./

target/debug/interpreter: FORCE
	cargo build

translate:

FORCE: ;
