pub(crate) type AccountId = sp_core::crypto::AccountId32;
pub(crate) type BlockNumber = u32;
pub(crate) type Moment = u64;
pub(crate) type Balance = u128;
pub(crate) type Header = sp_runtime::generic::Header<BlockNumber, sp_runtime::traits::BlakeTwo256>;
pub(crate) type Hash = sp_core::H256;
pub(crate) type Index = u32;

pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<AccountId, (), (), ()>;
pub type Block = sp_runtime::generic::Block<sp_runtime::testing::Header, UncheckedExtrinsic>;

pub const DOTS: Balance = 1_000_000_000_000;
pub const DOLLARS: Balance = DOTS / 6;
pub const CENTS: Balance = DOLLARS / 100;
pub const MILLICENTS: Balance = CENTS / 1_000;

pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

impl frame_system::Config for Runtime {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = Hash;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = ();
	type Version = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type PalletInfo = PalletInfo;
	type SS58Prefix = ();
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type MaxLocks = ();
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ();
	type AccountStore = frame_system::Module<Runtime>;
	type WeightInfo = ();
}

frame_support::parameter_types! {
	pub const CandidateDeposit: Balance = 10 * DOLLARS;
	pub const WrongSideDeduction: Balance = 2 * DOLLARS;
	pub const MaxStrikes: u32 = 10;
	pub const RotationPeriod: BlockNumber = 80 * HOURS;
	pub const PeriodSpend: Balance = 500 * DOLLARS;
	pub const MaxLockDuration: BlockNumber = 36 * 30 * DAYS;
	pub const ChallengePeriod: BlockNumber = 7 * DAYS;
	pub const SocietyModuleId: sp_runtime::ModuleId = sp_runtime::ModuleId(*b"py/socie");
}

impl pallet_society::Config for Runtime {
	type Event = ();
	type Currency = pallet_balances::Module<Self>;
	type Randomness = frame_support::traits::TestRandomness;
	type CandidateDeposit = CandidateDeposit;
	type WrongSideDeduction = WrongSideDeduction;
	type MaxStrikes = MaxStrikes;
	type PeriodSpend = PeriodSpend;
	type MembershipChanged = ();
	type RotationPeriod = RotationPeriod;
	type MaxLockDuration = MaxLockDuration;
	type FounderSetOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type SuspensionJudgementOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type ChallengePeriod = ChallengePeriod;
	type ModuleId = SocietyModuleId;
}

frame_support::construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Event<T>},
		Balances: pallet_balances::{Module, Call, Event<T>, Config<T>},
		Society: pallet_society::{Module, Call, Event<T>, Config<T>},
	}
);

#[tokio::main]
async fn main() -> () {
	let _ = env_logger::Builder::from_default_env()
		.format_module_path(true)
		.format_level(true)
		.try_init();

	sp_core::crypto::set_default_ss58_version(sp_core::crypto::Ss58AddressFormat::KusamaAccount);

	#[allow(dead_code)]
	let offline_mode = remote_externalities::Mode::Offline(remote_externalities::OfflineConfig {
		cache: remote_externalities::CacheConfig {
			directory: ".".to_string(),
			name: "kusama@6480000".to_string(),
		},
	});

	#[allow(dead_code)]
	let online_mode = remote_externalities::Mode::Online(remote_externalities::OnlineConfig {
		at: Some(
			hex_literal::hex!["45ca77cabc7dc26951b6cde4037812044d57c3d755210f6aff48f3ce0882f984"]
				// parent of https://polkascan.io/kusama/block/6480000
				.into(),
		),
		uri: "http://substrate-archive-0.parity-vpn.parity.io:9933/".to_string(),
		cache: Some(remote_externalities::CacheConfig {
			directory: ".".to_string(),
			name: "kusama@6480000".to_string(),
		}),
		modules: vec!["System".to_string(), "Balances".to_string(), "Society".to_string()],
		..Default::default()
	});

	remote_externalities::Builder::default()
		.mode(offline_mode)
		.build()
		.await
		.unwrap()
		.execute_with(|| {
			let mut members = Society::members();
			Society::rotate_period(&mut members);
		});
}
