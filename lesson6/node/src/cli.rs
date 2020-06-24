use sc_cli::{RunCmd, Subcommand};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[structopt(flatten)]
	pub run: RunCmd,
}

/*src/cli.rs借助 StructOpt 库将命令行参数解析为Cli结构体，包含：

可选的子命令，如 purge-chain 清空本地存储， build-spec 创建一个spec.json的初始文件， 
revert 回滚链上状态等；
命令行参数，如 --validator 开启验证人模式， --light 以轻客户端方式运行， 
--ws-port 9944 指定WebSocket监听的TCP端口，等等。编译node-template之后，
可以通过 ./target/release/node-template -h 获取所有可用的子命令和参数，及其帮助信息。
*/