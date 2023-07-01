#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod healthcare_contract {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        traits::{PackedLayout, SpreadLayout},
    };

    #[ink(storage)]
    pub struct HealthcareContract {
        owner: AccountId,
        medical_records: StorageHashMap<(AccountId, AccountId), MedicalRecord>,
    }

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct MedicalRecord {
        data: [u8; 32],
    }

    impl HealthcareContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                medical_records: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn grant_access(&mut self, provider: AccountId) {
            let patient = Self::env().caller();
            let medical_record = MedicalRecord { data: [0u8; 32] };
            self.medical_records.insert((patient, provider), medical_record);
        }

        #[ink(message)]
        pub fn revoke_access(&mut self, provider: AccountId) {
            let patient = Self::env().caller();
            self.medical_records.take(&(patient, provider));
        }

        #[ink(message)]
        pub fn update_medical_record(&mut self, provider: AccountId, data: [u8; 32]) {
            let patient = Self::env().caller();
            let medical_record = self.medical_records.get_mut(&(patient, provider));
            if let Some(record) = medical_record {
                record.data = data;
            }
        }

        #[ink(message)]
        pub fn get_medical_record(&self, provider: AccountId) -> Option<[u8; 32]> {
            let patient = Self::env().caller();
            self.medical_records.get(&(patient, provider)).map(|record| record.data)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_grant_and_revoke_access() {
            let mut contract = HealthcareContract::new();
            let provider = AccountId::from([0x1; 32]);
            contract.grant_access(provider);
            assert_eq!(contract.get_medical_record(provider), Some([0u8; 32]));
            contract.revoke_access(provider);
            assert_eq!(contract.get_medical_record(provider), None);
        }

        #[ink::test]
        fn test_update_medical_record() {
            let mut contract = HealthcareContract::new();
            let provider = AccountId::from([0x1; 32]);
            let data = [0x41; 32];
            contract.grant_access(provider);
            contract.update_medical_record(provider, data);
            assert_eq!(contract.get_medical_record(provider), Some(data));
        }
    }
}
