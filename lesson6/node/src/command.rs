// This file is part of Substrate.

// Copyright (C) 2017-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::chain_spec;
use crate::cli::Cli;
use crate::service;
use sc_cli::SubstrateCli;

impl SubstrateCli for Cli {
	fn impl_name() -> &'static str {
		"Substrate Node"
	}

	fn impl_version() -> &'static str {
		env!("SUBSTRATE_CLI_IMPL_VERSION")
	}

	fn description() -> &'static str {
		env!("CARGO_PKG_DESCRIPTION")
	}

	fn author() -> &'static str {
		env!("CARGO_PKG_AUTHORS")
	}

	fn support_url() -> &'static str {
		"support.anonymous.an"
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn executable_name() -> &'static str {
		env!("CARGO_PKG_NAME")
	}
//通过 chain_spec::load_spec 获取chain的配置，来更新前面构造的Substrate服务配置
	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()),
			"" | "local" => Box::new(chain_spec::local_testnet_config()),
			path => Box::new(chain_spec::ChainSpec::from_json_file(
				std::path::PathBuf::from(path),
			)?),
		})
	}
}

/// Parse and run command line arguments
/*通过 from_args 解析命令行的执行参数，返回一个 Cli 结构体，具体参考下面src/cli.rs的内容；

创建一个默认的 Substrate服务配置 ，这些服务包含启动线程运行网络、客户端和交易池等；

如果返回的 Cli 实例里存在子命令，则执行子命令，执行子命令时，

首先进行初始化，如设置panic的异常处理机制，日志等；

通过 chain_spec::load_spec 获取chain的配置，来更新前面构造的Substrate服务配置；

调用子命令的 run 函数来执行该命令，run函数依赖src/service.rs模块里的 new_full_start 宏来返回ServiceBuilder，包含了构建Substrate服务的多种组件。

如果返回的 Cli 实例里没有子命令，则执行当前命令，

首先初始化，和子命令的初始化功能一样；

更新Substrate服务配置，比子命令的更新操作更全面，配置的所有属性都会更新；

调用 run 来启动节点，需要传入全节点客户端的服务实例和轻节点客户端的服务实例，
根据服务配置中的节点角色进行选择，启动完成后，保持运行直到接收到退出信号 SIGINT （即Ctrl+C）。
*/
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(subcommand) => {
			let runner = cli.create_runner(subcommand)?;
			runner.run_subcommand(subcommand, |config| Ok(new_full_start!(config).0))
		}
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node(
				service::new_light,
				service::new_full,
				node_template_runtime::VERSION
			)
		}
	}
}
