#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure};
use frame_system::{self as system, ensure_signed};
use smart_contracts::{SmartContract, Call};

pub trait Trait: system::Trait {
    // Add your own trait definitions here
}

decl_storage! {
    trait Store for Module<T: Trait> as Example {
        // Declare storage items here
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Declare module events here
        fn call(origin, call: Call) -> Result {
            let sender = ensure_signed(origin)?;

            // Call the smart contract
            let result = SmartContract::call(call, sender);

            // Check the result of the call and emit an event if successful
            if let Ok(_) = result {
                Self::deposit_event(RawEvent::CallSuccessful(sender, call));
            }

            Ok(())
        }
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        Call = Call,
    {
        CallSuccessful(AccountId, Call),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        // Declare errors here
    }
}
