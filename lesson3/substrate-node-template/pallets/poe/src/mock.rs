// Creating mock runtime here

use crate::{Module, Trait};
use sp_core::H256;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight, impl_outer_event};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill,
};
use frame_system as system;
//为runtime构造了一个Origin类型，用来标识交易的来源；
impl_outer_origin! {
	pub enum Origin for Test {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;//创建了一个测试用的runtime结构体；

//生成一些后面功能模块所需的满足 Get 接口的数据类型；
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

// mod generic_event {
// 	pub use crate::Event;
// }
// impl_outer_event! {
// 	pub enum TestEvent for Test {
// 		generic_event<T>,
// 		system<T>,
// 	}
// }

parameter_types!{
	pub const MaxLenth:usize = 6;
	pub const MinLenth:u32 = 1;
}
//为runtime实现各个功能模块接口，这里使用了大量的 () 来mock不关心的数据类型；
impl Trait for Test {
	type Event = ();
	//type Event = TestEvent;
	type MaxLenthp = MaxLenth;
	type MinLenthp = MinLenth;
	
}
pub type PoeModule = Module<Test>;
//pub type System = system::Module<Test>;

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

// pub struct ExtBuilder;

// impl ExtBuilder {
// 	pub fn build() -> sp_io::TestExternalities {
// 		let storage = system::GenesisConfig::default()
// 			.build_storage::<Test>()
// 			.unwrap();
// 		let mut ext = sp_io::TestExternalities::from(storage);
// 		ext.execute_with(|| System::set_block_number(1));
// 		ext
// 	}
// }
