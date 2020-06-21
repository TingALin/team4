//! Substrate Node Template CLI library.
//在编译时，当模块缺少文档时会打印warnning
#![warn(missing_docs)]
//引入了当前目录下的其它代码模块
mod chain_spec;
#[macro_use] //会加载引入的模块下的所有宏
mod service;
mod cli;
mod command;
//main函数是程序的入口，它返回一个 自定义的Result类型 ，在函数内首先构造了一个 VersionInfo 的结构体，
//用来保存可执行程序的版本信息，其中 VERGEN_SHA_SHORT 是在编译时由build.rs产生的，
//然后执行command模块提供的run函数。
fn main() -> sc_cli::Result<()> {
	command::run()
}
