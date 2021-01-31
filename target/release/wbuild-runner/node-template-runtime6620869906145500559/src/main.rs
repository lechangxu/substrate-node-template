
				//! This is automatically generated code by `substrate-wasm-builder`.

				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/home/ubuntu/substrate-node-template/target/release/build/node-template-runtime-72ad8c0cc3cfc62f/out/wasm_binary.rs",
						"/home/ubuntu/substrate-node-template/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			