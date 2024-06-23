use pbc_contract_common::{
    address::{Address, Shortname},
    events::EventGroupBuilder,
};

pub struct CommonContract {
    contract_address: Address,
}

impl CommonContract {
    const SHORTNAME_VERIFY: Shortname = Shortname::from_u32(0x01);

    pub fn at_address(contract_address: Address) -> Self {
        Self { contract_address }
    }

    pub fn verify(&self, event_group_builder: &mut EventGroupBuilder, bool: bool) {
        event_group_builder
            .call(self.contract_address, Self::SHORTNAME_VERIFY)
            .argument(bool)
            .done()
    }
}
