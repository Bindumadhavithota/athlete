#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

mod crypto;
mod mock;
mod offchain;
mod tests;

pub use crypto::*;
pub use offchain::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use alloc::vec::Vec;
  use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure,
    pallet_prelude::{StorageMap, StorageValue},
    traits::{
      tokens::nonfungibles::{Create, Inspect, InspectEnumerable, Mutate, Transfer},
      Currency, ExistenceRequirement, Get, Hooks, IsType,
    },
    Twox64Concat,
  };
  use frame_system::{
    ensure_root, ensure_signed,
    offchain::{AppCrypto, SendTransactionTypes, SigningTypes},
    pallet_prelude::{BlockNumberFor, OriginFor},
  };
  use meta_athlete_primitives::{
    Athlete, AthleteCardAttributes, AthleteCardClass, ClassId, InstanceId,
  };
  use sp_runtime::ArithmeticError;

  type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn mint_cards(origin: OriginFor<T>, athlete: Athlete) -> DispatchResult {
      let who = ensure_signed(origin)?;

      ensure!(
        Athletes::<T>::iter().all(|elem| &elem.1 != &athlete),
        Error::<T>::AthleteAlreadyExists
      );

      let id = Self::next_athlete_id()?;
      <Athletes<T>>::insert(id, &athlete);

      T::Card::create_class(&id, &who, &who)?;
      Self::set_class_attributes(&id, &athlete)?;

      let add_instances = |range, ty| {
        for idx in range {
          T::Card::mint_into(&id, &idx, &who)?;
          Self::set_attributes(
            &AthleteCardAttributes {
              price: None,
              total_shares: <_>::from(0u8),
              ty,
              views: <_>::from(0u8),
              votes: <_>::from(0u8),
            },
            &id,
            &idx,
          )?;
        }
        DispatchResult::Ok(())
      };

      add_instances(0..100, AthleteCardClass::Silver.into())?;
      add_instances(100..140, AthleteCardClass::Gold.into())?;
      add_instances(140..150, AthleteCardClass::Platinum.into())?;

      Ok(())
    }

    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn set_card_price(
      origin: OriginFor<T>,
      class_id: ClassId,
      instance_id: InstanceId,
      price: Option<BalanceOf<T>>,
    ) -> DispatchResult {
      let who = ensure_signed(origin)?;
      let owner = Self::owner(&class_id, &instance_id)?;
      ensure!(owner == who, Error::<T>::MustBeCardOwner);
      Self::set_price(&class_id, &instance_id, &price)?;
      Ok(())
    }

    #[pallet::weight(10_000)]
    pub fn submit_athlete_info(
      origin: OriginFor<T>,
      class_id: ClassId,
      instance_id: InstanceId,
      views: u32,
      votes: u32,
      _signature: T::Signature,
    ) -> DispatchResult {
      ensure_root(origin)?;
      Self::set_views(&class_id, &instance_id, &views)?;
      Self::set_votes(&class_id, &instance_id, &votes)?;
      Ok(())
    }

    #[frame_support::transactional]
    #[pallet::weight(10_000)]
    pub fn transfer_card(
      origin: OriginFor<T>,
      class_id: ClassId,
      instance_id: InstanceId,
    ) -> DispatchResult {
      let destination = ensure_signed(origin)?;
      let source = Self::owner(&class_id, &instance_id)?;
      let price = if let Some(elem) = Self::price(&class_id, &instance_id) {
        elem
      } else {
        return Err(Error::<T>::CardIsNotForSale.into());
      };
      T::Currency::transfer(&destination, &source, price, ExistenceRequirement::AllowDeath)?;
      T::Card::transfer(&class_id, &instance_id, &destination)?;
      Ok(())
    }
  }

  #[pallet::config]
  pub trait Config: SendTransactionTypes<Call<Self>> + SigningTypes + frame_system::Config {
    type Card: Create<Self::AccountId, ClassId = ClassId, InstanceId = InstanceId>
      + InspectEnumerable<Self::AccountId, ClassId = ClassId, InstanceId = InstanceId>
      + Mutate<Self::AccountId, ClassId = ClassId, InstanceId = InstanceId>
      + Transfer<Self::AccountId, ClassId = ClassId, InstanceId = InstanceId>;
    type Currency: Currency<Self::AccountId>;
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    type OffchainAuthority: AppCrypto<Self::Public, Self::Signature>;
    type OffchainUnsignedGracePeriod: Get<Self::BlockNumber>;
    type OffchainUnsignedInterval: Get<Self::BlockNumber>;
  }

  #[pallet::error]
  pub enum Error<T> {
    AthleteAlreadyExists,
    CardAttributeDoesNotExist,
    CardDoesNotHaveAnOwner,
    CardIsNotForSale,
    MustBeCardOwner,
  }

  #[pallet::event]
  pub enum Event<T: Config> {
    SomethingStored(u32, T::AccountId),
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(_: T::BlockNumber) {
      let _ = Self::fetch_pair_prices_and_submit_tx();
    }
  }

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  pub type AthleteCounter<T: Config> = StorageValue<_, ClassId>;

  #[pallet::storage]
  pub type Athletes<T: Config> = StorageMap<_, Twox64Concat, ClassId, Athlete>;

  impl<T> Pallet<T>
  where
    T: Config,
  {
    create_typed_getter_and_setter!(@optional price, set_price, BalanceOf<T>);

    create_typed_getter_and_setter!(@mandatory total_shares, set_total_shares, u32);

    create_typed_getter_and_setter!(@mandatory ty, set_ty, AthleteCardClass);

    create_typed_getter_and_setter!(@mandatory views, set_views, u32);

    create_typed_getter_and_setter!(@mandatory votes, set_votes, u32);

    create_typed_class_getter_and_setter!(@mandatory class_height, set_class_height, u16);

    create_typed_class_getter_and_setter!(@mandatory class_name, set_class_name, Vec<u8>);

    create_typed_class_getter_and_setter!(@mandatory class_photo, set_class_photo, Vec<u8>);

    create_typed_class_getter_and_setter!(@mandatory class_weight, set_class_weight, u16);

    #[cfg(test)]
    pub(crate) fn attributes(
      class_id: &ClassId,
      instance_id: &InstanceId,
    ) -> Result<AthleteCardAttributes<BalanceOf<T>>, DispatchError> {
      Ok(AthleteCardAttributes {
        price: Self::price(class_id, instance_id),
        total_shares: Self::total_shares(class_id, instance_id)?,
        ty: Self::ty(class_id, instance_id)?,
        views: Self::views(class_id, instance_id)?,
        votes: Self::votes(class_id, instance_id)?,
      })
    }

    #[cfg(test)]
    pub(crate) fn class_attributes(class_id: &ClassId) -> Result<Athlete, DispatchError> {
      Ok(Athlete {
        height: Self::class_height(class_id)?,
        name: Self::class_name(class_id)?,
        photo: Self::class_photo(class_id)?,
        weight: Self::class_weight(class_id)?,
      })
    }

    pub(crate) fn owner(
      class_id: &ClassId,
      instance_id: &InstanceId,
    ) -> Result<T::AccountId, DispatchError> {
      T::Card::owner(class_id, instance_id).ok_or_else(|| Error::<T>::CardDoesNotHaveAnOwner.into())
    }

    fn next_athlete_id() -> Result<ClassId, DispatchError> {
      let id = if let Ok(current) = AthleteCounter::<T>::try_get() {
        current.checked_add(1u128).ok_or(ArithmeticError::Overflow)?
      } else {
        1
      };
      <AthleteCounter<T>>::put(id);
      Ok(id)
    }

    fn set_attributes(
      attributes: &AthleteCardAttributes<BalanceOf<T>>,
      class_id: &ClassId,
      instance_id: &InstanceId,
    ) -> DispatchResult {
      let AthleteCardAttributes { ref price, ref total_shares, ref ty, ref views, ref votes } =
        *attributes;
      Self::set_price(class_id, instance_id, price)?;
      Self::set_total_shares(class_id, instance_id, total_shares)?;
      Self::set_ty(class_id, instance_id, ty)?;
      Self::set_views(class_id, instance_id, views)?;
      Self::set_votes(class_id, instance_id, votes)?;
      Ok(())
    }

    fn set_class_attributes(class_id: &ClassId, class_attributes: &Athlete) -> DispatchResult {
      let Athlete { ref height, ref name, ref photo, ref weight } = *class_attributes;
      Self::set_class_height(class_id, height)?;
      Self::set_class_name(class_id, name)?;
      Self::set_class_photo(class_id, photo)?;
      Self::set_class_weight(class_id, weight)?;
      Ok(())
    }
  }
}
