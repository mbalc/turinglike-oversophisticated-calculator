all: interpreter translate

interpreter: target/debug/interpreter
	cp $^ ./


translate: target/debug/translate
	cp $^ ./

target/debug/interpreter: FORCE_BUILD
target/debug/translate: FORCE_BUILD

FORCE_BUILD:
	cargo build
