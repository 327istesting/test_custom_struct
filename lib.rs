#![cfg_attr(not(feature = "std"), no_std)]


#[ink::contract]
mod test_erc {

    use ink::storage::Mapping;

     // The Meta_Defender result types.
    pub type Result<T> = core::result::Result<T, Error>;


    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        NotExist,
        NotFalse
    }

    #[derive(scale::Encode, scale::Decode, Debug, PartialEq, Eq,)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    #[derive(Default)]
    struct CustomBool {
        boolean: bool,
    }


    #[ink(storage)]
    pub struct Test {
        count: u128,
        mappings: Mapping<u128, CustomBool>,
    }

    impl Test{

        #[ink(constructor)]
        pub fn new(
        ) -> Self {
            let mappings = Default::default();
            Self { 
                count:0,
                mappings
            }
        }

        #[ink(message)]
        pub fn insert_value (&mut self, boolean: bool) -> Result<()>{
            let mut c = CustomBool{boolean};
            self.mappings.insert(self.count,  &mut c);
            self.count += 1;
            Ok(())
        }


        #[ink(message)]
        pub fn change_value (&mut self, id: u128) -> Result<()>{
            match self.mappings.get(id){
                None => return Err(Error::NotExist),
                Some(mut v) =>{
                    if v.boolean == true {
                        return Err(Error::NotFalse);
                    }else{
                        ink::env::debug_println!("before change, boolean is: {}", v.boolean);
                        v.boolean = true;
                        ink::env::debug_println!("after change, boolean is: {}", v.boolean);
                    }
                }
            }
            match self.mappings.get(id){
                None => return Err(Error::NotExist),
                Some(v) =>{
                    ink::env::debug_println!("outside first match, boolean is: {}", v.boolean);
                }
            }
            Ok(())
        }



    }


}
