use wasm_builder_runner::WasmBuilder;

fn main() {
	WasmBuilder::new()
		.with_current_project()
		.with_wasm_builder_from_crates("1.0.9")
		.export_heap_base()
		.import_memory()
		.build()
}

/*
自定义的构建脚本放置在项目的build.rs文件内，可以在编译构建项目之前，让Cargo去编译和执行该脚本，使用场景有：

编译、连接第三方的非Rust代码；
构建之前的代码生成功能
https://docs.rs/substrate-wasm-builder-runner/1.0.6/substrate_wasm_builder_runner/struct.WasmBuilder.html#method.import_memory
The builder for building a wasm binary.

The builder itself is seperated into multiple structs to make the setup type safe.

Building a wasm binary:

Call WasmBuilder::new to create a new builder.
Select the project to build using the methods of WasmBuilderSelectProject.
Select the source of the wasm-builder crate using the methods of WasmBuilderSelectSource.
Set additional RUST_FLAGS or a different name for the file containing the WASM code using methods of WasmBuilder.
Build the WASM binary using Self::build.

build.rs使用 wasm-builder-runner 将当前的runtime项目编译为Wasm，编译后的文件位于 target/release/wbuild/node-template-runtime/node_template_runtime.compact.wasm
*/