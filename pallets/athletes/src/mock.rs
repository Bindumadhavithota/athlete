#![cfg(test)]

use crate as pallet_athletes;
use frame_support::traits::{ConstU128, ConstU32, ConstU64};
use frame_system::offchain::AppCrypto;
use meta_athlete_primitives::{Balance, InstanceId};
use sp_core::{sr25519, H256};
use sp_runtime::{
  testing::{Header, TestXt},
  traits::{BlakeTwo256, IdentityLookup, Verify},
};

pub(crate) type AccountId = sr25519::Public;
pub(crate) type Block = frame_system::mocking::MockBlock<Runtime>;
pub(crate) type Extrinsic = TestXt<crate::Call<Runtime>, ()>;
pub(crate) type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;

frame_support::construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = Block,
    UncheckedExtrinsic = UncheckedExtrinsic,
  {
    Athletes: pallet_athletes::{Pallet, Call, Storage, Event<T>},
    Balances: pallet_balances::{Call, Config<T>, Event<T>, Pallet, Storage},
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
  }
);

impl pallet_athletes::Config for Runtime {
  type Card = Uniques;
  type Currency = Balances;
  type Event = Event;
  type OffchainAuthority = TestAuth;
  type OffchainUnsignedGracePeriod = ConstU64<5>;
  type OffchainUnsignedInterval = ConstU64<128>;
}

impl pallet_balances::Config for Runtime {
  type AccountStore = System;
  type Balance = Balance;
  type DustRemoval = ();
  type Event = Event;
  type ExistentialDeposit = ConstU128<1>;
  type MaxLocks = ();
  type MaxReserves = ();
  type ReserveIdentifier = [u8; 8];
  type WeightInfo = ();
}

impl pallet_uniques::Config for Runtime {
  type AttributeDepositBase = ConstU128<1>;
  type ClassDeposit = ConstU128<2>;
  type ClassId = u128;
  type Currency = Balances;
  type DepositPerByte = ConstU128<1>;
  type Event = Event;
  type ForceOrigin = frame_system::EnsureRoot<AccountId>;
  type InstanceDeposit = ConstU128<1>;
  type InstanceId = InstanceId;
  type KeyLimit = ConstU32<50>;
  type MetadataDepositBase = ConstU128<1>;
  type StringLimit = ConstU32<50>;
  type ValueLimit = ConstU32<2048>;
  type WeightInfo = ();
}

impl frame_system::Config for Runtime {
  type AccountData = pallet_balances::AccountData<Balance>;
  type AccountId = AccountId;
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockHashCount = ConstU64<250>;
  type BlockLength = ();
  type BlockNumber = u64;
  type BlockWeights = ();
  type Call = Call;
  type DbWeight = ();
  type Event = Event;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type Header = Header;
  type Index = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type MaxConsumers = ConstU32<16>;
  type OnKilledAccount = ();
  type OnNewAccount = ();
  type OnSetCode = ();
  type Origin = Origin;
  type PalletInfo = PalletInfo;
  type SS58Prefix = ();
  type SystemWeightInfo = ();
  type Version = ();
}

impl<LC> frame_system::offchain::SendTransactionTypes<LC> for Runtime
where
  crate::Call<Runtime>: From<LC>,
{
  type Extrinsic = Extrinsic;
  type OverarchingCall = crate::Call<Runtime>;
}

impl frame_system::offchain::SigningTypes for Runtime {
  type Public = <sr25519::Signature as Verify>::Signer;
  type Signature = sr25519::Signature;
}

pub struct TestAuth;

impl AppCrypto<<sr25519::Signature as Verify>::Signer, sr25519::Signature> for TestAuth {
  type GenericPublic = sr25519::Public;
  type GenericSignature = sr25519::Signature;
  type RuntimeAppPublic = crate::Public;
}

pub(crate) fn test_ext() -> sp_io::TestExternalities {
  let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

  pallet_balances::GenesisConfig::<Runtime> { balances: vec![(alice(), 1000), (bob(), 1000)] }
    .assimilate_storage(&mut t)
    .unwrap();

  t.into()
}

pub(crate) fn alice() -> sr25519::Public {
  <sr25519::Public>::from_raw({
    let mut array = [0; 32];
    array[31] = 1;
    array
  })
}

pub(crate) fn bob() -> sr25519::Public {
  <sr25519::Public>::from_raw({
    let mut array = [0; 32];
    array[31] = 2;
    array
  })
}
