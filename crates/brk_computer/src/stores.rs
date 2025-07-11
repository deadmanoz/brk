use std::path::Path;

use brk_core::{
    OutputIndex, P2AAddressIndex, P2MSOutputIndex, P2PK33AddressIndex, P2PK65AddressIndex,
    P2PKHAddressIndex, P2SHAddressIndex, P2TRAddressIndex, P2WPKHAddressIndex, P2WSHAddressIndex,
    Version,
};
use brk_store::Store;
use fjall::TransactionalKeyspace;
use jiff::Unit;

const _VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Stores {
    pub p2aaddressindex_to_addressdata: Store<(P2AAddressIndex, OutputIndex), Unit>,
    pub p2aaddressindex_to_emptyaddressdata: Store<(P2AAddressIndex, OutputIndex), Unit>,
    pub p2aaddressindex_to_utxos_received: Store<(P2AAddressIndex, OutputIndex), Unit>,
    pub p2aaddressindex_to_utxos_sent: Store<P2AAddressIndex, Unit>,

    pub p2msoutputindex_to_addressdata: Store<P2MSOutputIndex, Unit>,
    pub p2msoutputindex_to_emptyaddressdata: Store<P2MSOutputIndex, Unit>,
    pub p2msoutputindex_to_utxos_received: Store<P2MSOutputIndex, Unit>,
    pub p2msoutputindex_to_utxos_sent: Store<P2MSOutputIndex, Unit>,

    pub p2pk33addressindex_to_addressdata: Store<P2PK33AddressIndex, Unit>,
    pub p2pk33addressindex_to_emptyaddressdata: Store<P2PK33AddressIndex, Unit>,
    pub p2pk33addressindex_to_utxos_received: Store<P2PK33AddressIndex, Unit>,
    pub p2pk33addressindex_to_utxos_sent: Store<P2PK33AddressIndex, Unit>,

    pub p2pk65addressindex_to_addressdata: Store<P2PK65AddressIndex, Unit>,
    pub p2pk65addressindex_to_emptyaddressdata: Store<P2PK65AddressIndex, Unit>,
    pub p2pk65addressindex_to_utxos_received: Store<P2PK65AddressIndex, Unit>,
    pub p2pk65addressindex_to_utxos_sent: Store<P2PK65AddressIndex, Unit>,

    pub p2pkhaddressindex_to_addressdata: Store<P2PKHAddressIndex, Unit>,
    pub p2pkhaddressindex_to_emptyaddressdata: Store<P2PKHAddressIndex, Unit>,
    pub p2pkhaddressindex_to_utxos_received: Store<P2PKHAddressIndex, Unit>,
    pub p2pkhaddressindex_to_utxos_sent: Store<P2PKHAddressIndex, Unit>,

    pub p2shaddressindex_to_addressdata: Store<P2SHAddressIndex, Unit>,
    pub p2shaddressindex_to_emptyaddressdata: Store<P2SHAddressIndex, Unit>,
    pub p2shaddressindex_to_utxos_received: Store<P2SHAddressIndex, Unit>,
    pub p2shaddressindex_to_utxos_sent: Store<P2SHAddressIndex, Unit>,

    pub p2traddressindex_to_addressdata: Store<P2TRAddressIndex, Unit>,
    pub p2traddressindex_to_emptyaddressdata: Store<P2TRAddressIndex, Unit>,
    pub p2traddressindex_to_utxos_received: Store<P2TRAddressIndex, Unit>,
    pub p2traddressindex_to_utxos_sent: Store<P2TRAddressIndex, Unit>,

    pub p2wpkhaddressindex_to_addressdata: Store<P2WPKHAddressIndex, Unit>,
    pub p2wpkhaddressindex_to_emptyaddressdata: Store<P2WPKHAddressIndex, Unit>,
    pub p2wpkhaddressindex_to_utxos_received: Store<P2WPKHAddressIndex, Unit>,
    pub p2wpkhaddressindex_to_utxos_sent: Store<P2WPKHAddressIndex, Unit>,

    pub p2wshaddressindex_to_addressdata: Store<P2WSHAddressIndex, Unit>,
    pub p2wshaddressindex_to_emptyaddressdata: Store<P2WSHAddressIndex, Unit>,
    pub p2wshaddressindex_to_utxos_received: Store<P2WSHAddressIndex, Unit>,
    pub p2wshaddressindex_to_utxos_sent: Store<P2WSHAddressIndex, Unit>,
}

impl Stores {
    pub fn import(_: &Path, _: Version, _: &TransactionalKeyspace) -> color_eyre::Result<Self> {
        // let address_to_utxos_received = Store::import(
        //     keyspace.clone(),
        //     path,
        //     "address_to_utxos_received",
        //     version + VERSION + Version::ZERO,
        // )?;
        // let address_to_utxos_spent = Store::import(
        //     keyspace.clone(),
        //     path,
        //     "address_to_utxos_spent",
        //     version + VERSION + Version::ZERO,
        // )?;

        todo!();
        // Ok(Self {
        //     p2aaddressindex_to_utxos_received: Store::import(
        //         keyspace,
        //         path,
        //         name,
        //         version,
        //         bloom_filter_bits,
        //     ), // address_to_utxos_received,
        //        // address_to_utxos_spent,
        // })
    }
}
