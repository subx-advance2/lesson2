#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{StorageMap, StorageValue, decl_error, decl_event, decl_module, decl_storage, ensure, sp_std, traits::Randomness};
use frame_system::{ensure_signed};
use sp_io::hashing::blake2_128;
use sp_runtime::DispatchError;

type KittyIndex = u32;

#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Randomness: Randomness<Self::Hash>;
}

decl_storage! {
    trait Store for Module<T: Trait> as Kitties {
        pub Kitties get(fn kitties): map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
        pub KittiesCount get(fn kitties_count): KittyIndex;
        pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<T::AccountId>;
    }
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        KittiesCountOverflow,
        InvalidKittyId,
        RequireDifferentParent,
    }
}

decl_event! {
    pub enum Event<T> where <T as frame_system::Trait>::AccountId, {
        Created(AccountId, KittyIndex),
        Transferred(AccountId, AccountId, KittyIndex),
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = 0]
        pub fn create(origin) {
            let sender = ensure_signed(origin)?;
            let kitty_id =  Self::next_kitty_id()?;
            let dna = Self::random_value(&sender);
            
            let kitty = Kitty(dna);

            Self::insert_kitty(&sender, kitty_id, kitty);
            Self::deposit_event(RawEvent::Created(sender, kitty_id));
        }

        #[weight = 0]
        pub fn transfer(origin, to: T::AccountId, kitty_id: KittyIndex) {
            let sender = ensure_signed(origin)?;
            
            <KittyOwners<T>>::insert(kitty_id, to.clone());
            Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
        }

        #[weight = 0]
        pub fn breed(origin, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) {
            let sender = ensure_signed(origin)?;
            let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
            
            Self::deposit_event(RawEvent::Created(sender, new_kitty_id));
        }
    }
}

impl<T: Trait> Module<T> {

    fn insert_kitty(owner: &T::AccountId, kitty_id: KittyIndex, kitty: Kitty) {
        Kitties::insert(kitty_id, kitty);
        KittiesCount::put(kitty_id + 1);
        <KittyOwners<T>>::insert(kitty_id, owner);
    }

    fn next_kitty_id() -> sp_std::result::Result<KittyIndex, DispatchError> {
        let kitty_id = Self::kitties_count();
        if kitty_id == KittyIndex::max_value() {
            return Err(Error::<T>::KittiesCountOverflow.into());
        }

        Ok(kitty_id)
    }

    fn do_breed(sender: &T::AccountId, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> sp_std::result::Result<KittyIndex, DispatchError> {
        let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
        let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

        ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

        let kitty_id = Self::next_kitty_id()?;

        let kitty1_dna = kitty1.0;
        let kitty2_dna = kitty2.0;

        let selector = Self::random_value(&sender);
        let mut new_dna = [0u8; 16];

        for i in 0..kitty1_dna.len() {
            new_dna[i] = Self::combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
        }

        Self::insert_kitty(sender, kitty_id, Kitty(new_dna));

        Ok(kitty_id)
    }

    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Module<T>>::extrinsic_index(),
        );

        payload.using_encoded(blake2_128)
    }

    fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
        (selector & dna1) | (!selector & dna2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sp_core::H256;
    use frame_support::{impl_outer_origin, parameter_types, weights::Weight, traits::{OnFinalize, OnInitialize}};
    use frame_system as system;
    use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, testing::Header, Perbill};

    impl_outer_origin!{
        pub enum Origin for Test {}
    }

    #[derive(Clone, Eq, PartialEq, Debug)]
    pub struct Test;

    parameter_types! {
        pub const BlockHashCount: u64 = 250;
        pub const MaximumBlockWeight: Weight = 1024;
        pub const MaximumBlockLength: u32 = 2 * 1024;
        pub const AvaiableBlockRatio: Perbill = Perbill::from_percent(75);
    }

    impl system::Trait for Test {
        type Origin = Origin;
        type BaseCallFilter = ();
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
        type AvailableBlockRatio = AvaiableBlockRatio;
        type Version = ();
        type SystemWeightInfo = ();
        type PalletInfo = ();
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
    }

    type Randomness = pallet_randomness_collective_flip::Module<Test>;

    impl Trait for Test {
        type Event = ();
        type Randomness = Randomness;
    }

    pub type Kitties = Module<Test>;
    pub type System = frame_system::Module<Test>;

    fn new_test_ext() -> sp_io::TestExternalities {
        system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
    }

    fn run_to_block(n: u64) {
        while System::block_number() < n {
            let block_number = System::block_number();

            Kitties::on_finalize(block_number);
            System::on_finalize(block_number);
            System::set_block_number(block_number + 1);

            let new_block_number = System::block_number(); 
            System::on_initialize(new_block_number);
            Kitties:on_initialize(new_block_number);
        }
    }

    #[test]
    fn owned_kitties_can_append_values() {
        new_test_ext().execute_with(|| {
            run_to_block(10);
            assert_eq!(Kitties::create(Origin::signed(1),), Ok(()));
        })
    }
}


























