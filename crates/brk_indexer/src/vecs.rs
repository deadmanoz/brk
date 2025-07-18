use std::path::Path;

use brk_core::{
    AddressBytes, BlockHash, EmptyOutputIndex, Height, InputIndex, OpReturnIndex, OutputIndex,
    OutputType, P2AAddressIndex, P2ABytes, P2MSOutputIndex, P2PK33AddressIndex, P2PK33Bytes,
    P2PK65AddressIndex, P2PK65Bytes, P2PKHAddressIndex, P2PKHBytes, P2SHAddressIndex, P2SHBytes,
    P2TRAddressIndex, P2TRBytes, P2WPKHAddressIndex, P2WPKHBytes, P2WSHAddressIndex, P2WSHBytes,
    RawLockTime, Result, Sats, StoredF64, StoredU32, StoredUsize, Timestamp, TxIndex, TxVersion,
    Txid, TypeIndex, UnknownOutputIndex, Version, Weight,
};
use brk_vec::{AnyCollectableVec, AnyIndexedVec, Format, IndexedVec};
use rayon::prelude::*;

use crate::Indexes;

const VERSION: Version = Version::ZERO;

#[derive(Clone)]
pub struct Vecs {
    pub emptyoutputindex_to_txindex: IndexedVec<EmptyOutputIndex, TxIndex>,
    pub height_to_blockhash: IndexedVec<Height, BlockHash>,
    pub height_to_difficulty: IndexedVec<Height, StoredF64>,
    pub height_to_first_emptyoutputindex: IndexedVec<Height, EmptyOutputIndex>,
    pub height_to_first_inputindex: IndexedVec<Height, InputIndex>,
    pub height_to_first_opreturnindex: IndexedVec<Height, OpReturnIndex>,
    pub height_to_first_outputindex: IndexedVec<Height, OutputIndex>,
    pub height_to_first_p2aaddressindex: IndexedVec<Height, P2AAddressIndex>,
    pub height_to_first_p2msoutputindex: IndexedVec<Height, P2MSOutputIndex>,
    pub height_to_first_p2pk33addressindex: IndexedVec<Height, P2PK33AddressIndex>,
    pub height_to_first_p2pk65addressindex: IndexedVec<Height, P2PK65AddressIndex>,
    pub height_to_first_p2pkhaddressindex: IndexedVec<Height, P2PKHAddressIndex>,
    pub height_to_first_p2shaddressindex: IndexedVec<Height, P2SHAddressIndex>,
    pub height_to_first_p2traddressindex: IndexedVec<Height, P2TRAddressIndex>,
    pub height_to_first_p2wpkhaddressindex: IndexedVec<Height, P2WPKHAddressIndex>,
    pub height_to_first_p2wshaddressindex: IndexedVec<Height, P2WSHAddressIndex>,
    pub height_to_first_txindex: IndexedVec<Height, TxIndex>,
    pub height_to_first_unknownoutputindex: IndexedVec<Height, UnknownOutputIndex>,
    /// Doesn't guarantee continuity due to possible reorgs
    pub height_to_timestamp: IndexedVec<Height, Timestamp>,
    pub height_to_total_size: IndexedVec<Height, StoredUsize>,
    pub height_to_weight: IndexedVec<Height, Weight>,
    /// If outputindex == Outputindex::MAX then it's coinbase
    pub inputindex_to_outputindex: IndexedVec<InputIndex, OutputIndex>,
    pub opreturnindex_to_txindex: IndexedVec<OpReturnIndex, TxIndex>,
    pub outputindex_to_outputtype: IndexedVec<OutputIndex, OutputType>,
    pub outputindex_to_typeindex: IndexedVec<OutputIndex, TypeIndex>,
    pub outputindex_to_value: IndexedVec<OutputIndex, Sats>,
    pub p2aaddressindex_to_p2abytes: IndexedVec<P2AAddressIndex, P2ABytes>,
    pub p2msoutputindex_to_txindex: IndexedVec<P2MSOutputIndex, TxIndex>,
    pub p2pk33addressindex_to_p2pk33bytes: IndexedVec<P2PK33AddressIndex, P2PK33Bytes>,
    pub p2pk65addressindex_to_p2pk65bytes: IndexedVec<P2PK65AddressIndex, P2PK65Bytes>,
    pub p2pkhaddressindex_to_p2pkhbytes: IndexedVec<P2PKHAddressIndex, P2PKHBytes>,
    pub p2shaddressindex_to_p2shbytes: IndexedVec<P2SHAddressIndex, P2SHBytes>,
    pub p2traddressindex_to_p2trbytes: IndexedVec<P2TRAddressIndex, P2TRBytes>,
    pub p2wpkhaddressindex_to_p2wpkhbytes: IndexedVec<P2WPKHAddressIndex, P2WPKHBytes>,
    pub p2wshaddressindex_to_p2wshbytes: IndexedVec<P2WSHAddressIndex, P2WSHBytes>,
    pub txindex_to_base_size: IndexedVec<TxIndex, StoredU32>,
    pub txindex_to_first_inputindex: IndexedVec<TxIndex, InputIndex>,
    pub txindex_to_first_outputindex: IndexedVec<TxIndex, OutputIndex>,
    pub txindex_to_is_explicitly_rbf: IndexedVec<TxIndex, bool>,
    pub txindex_to_rawlocktime: IndexedVec<TxIndex, RawLockTime>,
    pub txindex_to_total_size: IndexedVec<TxIndex, StoredU32>,
    pub txindex_to_txid: IndexedVec<TxIndex, Txid>,
    pub txindex_to_txversion: IndexedVec<TxIndex, TxVersion>,
    pub unknownoutputindex_to_txindex: IndexedVec<UnknownOutputIndex, TxIndex>,
}

impl Vecs {
    pub fn forced_import(path: &Path, version: Version) -> color_eyre::Result<Self> {
        Ok(Self {
            emptyoutputindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_blockhash: IndexedVec::forced_import(
                path,
                "blockhash",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_difficulty: IndexedVec::forced_import(
                path,
                "difficulty",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_emptyoutputindex: IndexedVec::forced_import(
                path,
                "first_emptyoutputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_inputindex: IndexedVec::forced_import(
                path,
                "first_inputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_opreturnindex: IndexedVec::forced_import(
                path,
                "first_opreturnindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_outputindex: IndexedVec::forced_import(
                path,
                "first_outputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2aaddressindex: IndexedVec::forced_import(
                path,
                "first_p2aaddressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2msoutputindex: IndexedVec::forced_import(
                path,
                "first_p2msoutputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2pk33addressindex: IndexedVec::forced_import(
                path,
                "first_p2pk33addressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2pk65addressindex: IndexedVec::forced_import(
                path,
                "first_p2pk65addressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2pkhaddressindex: IndexedVec::forced_import(
                path,
                "first_p2pkhaddressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2shaddressindex: IndexedVec::forced_import(
                path,
                "first_p2shaddressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2traddressindex: IndexedVec::forced_import(
                path,
                "first_p2traddressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2wpkhaddressindex: IndexedVec::forced_import(
                path,
                "first_p2wpkhaddressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_p2wshaddressindex: IndexedVec::forced_import(
                path,
                "first_p2wshaddressindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_txindex: IndexedVec::forced_import(
                path,
                "first_txindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_first_unknownoutputindex: IndexedVec::forced_import(
                path,
                "first_unknownoutputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_timestamp: IndexedVec::forced_import(
                path,
                "timestamp",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_total_size: IndexedVec::forced_import(
                path,
                "total_size",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            height_to_weight: IndexedVec::forced_import(
                path,
                "weight",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            inputindex_to_outputindex: IndexedVec::forced_import(
                path,
                "outputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            opreturnindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            outputindex_to_outputtype: IndexedVec::forced_import(
                path,
                "outputtype",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            outputindex_to_typeindex: IndexedVec::forced_import(
                path,
                "typeindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            outputindex_to_value: IndexedVec::forced_import(
                path,
                "value",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2aaddressindex_to_p2abytes: IndexedVec::forced_import(
                path,
                "p2abytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2msoutputindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2pk33addressindex_to_p2pk33bytes: IndexedVec::forced_import(
                path,
                "p2pk33bytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2pk65addressindex_to_p2pk65bytes: IndexedVec::forced_import(
                path,
                "p2pk65bytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2pkhaddressindex_to_p2pkhbytes: IndexedVec::forced_import(
                path,
                "p2pkhbytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2shaddressindex_to_p2shbytes: IndexedVec::forced_import(
                path,
                "p2shbytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2traddressindex_to_p2trbytes: IndexedVec::forced_import(
                path,
                "p2trbytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2wpkhaddressindex_to_p2wpkhbytes: IndexedVec::forced_import(
                path,
                "p2wpkhbytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            p2wshaddressindex_to_p2wshbytes: IndexedVec::forced_import(
                path,
                "p2wshbytes",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_base_size: IndexedVec::forced_import(
                path,
                "base_size",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_first_inputindex: IndexedVec::forced_import(
                path,
                "first_inputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_first_outputindex: IndexedVec::forced_import(
                path,
                "first_outputindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_is_explicitly_rbf: IndexedVec::forced_import(
                path,
                "is_explicitly_rbf",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_rawlocktime: IndexedVec::forced_import(
                path,
                "rawlocktime",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_total_size: IndexedVec::forced_import(
                path,
                "total_size",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_txid: IndexedVec::forced_import(
                path,
                "txid",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            txindex_to_txversion: IndexedVec::forced_import(
                path,
                "txversion",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
            unknownoutputindex_to_txindex: IndexedVec::forced_import(
                path,
                "txindex",
                version + VERSION + Version::ZERO,
                Format::Raw,
            )?,
        })
    }

    pub fn rollback_if_needed(&mut self, starting_indexes: &Indexes) -> Result<()> {
        let saved_height = starting_indexes.height.decremented().unwrap_or_default();

        let &Indexes {
            emptyoutputindex,
            height,
            inputindex,
            opreturnindex,
            outputindex,
            p2aaddressindex,
            p2msoutputindex,
            p2pk33addressindex,
            p2pk65addressindex,
            p2pkhaddressindex,
            p2shaddressindex,
            p2traddressindex,
            p2wpkhaddressindex,
            p2wshaddressindex,
            txindex,
            unknownoutputindex,
        } = starting_indexes;

        self.emptyoutputindex_to_txindex
            .truncate_if_needed(emptyoutputindex, saved_height)?;
        self.height_to_blockhash
            .truncate_if_needed(height, saved_height)?;
        self.height_to_difficulty
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_emptyoutputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_inputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_opreturnindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_outputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2aaddressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2msoutputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pk33addressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pk65addressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2pkhaddressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2shaddressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2traddressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2wpkhaddressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_p2wshaddressindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_txindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_first_unknownoutputindex
            .truncate_if_needed(height, saved_height)?;
        self.height_to_timestamp
            .truncate_if_needed(height, saved_height)?;
        self.height_to_total_size
            .truncate_if_needed(height, saved_height)?;
        self.height_to_weight
            .truncate_if_needed(height, saved_height)?;
        self.inputindex_to_outputindex
            .truncate_if_needed(inputindex, saved_height)?;
        self.opreturnindex_to_txindex
            .truncate_if_needed(opreturnindex, saved_height)?;
        self.outputindex_to_outputtype
            .truncate_if_needed(outputindex, saved_height)?;
        self.outputindex_to_typeindex
            .truncate_if_needed(outputindex, saved_height)?;
        self.outputindex_to_value
            .truncate_if_needed(outputindex, saved_height)?;
        self.p2aaddressindex_to_p2abytes
            .truncate_if_needed(p2aaddressindex, saved_height)?;
        self.p2msoutputindex_to_txindex
            .truncate_if_needed(p2msoutputindex, saved_height)?;
        self.p2pk33addressindex_to_p2pk33bytes
            .truncate_if_needed(p2pk33addressindex, saved_height)?;
        self.p2pk65addressindex_to_p2pk65bytes
            .truncate_if_needed(p2pk65addressindex, saved_height)?;
        self.p2pkhaddressindex_to_p2pkhbytes
            .truncate_if_needed(p2pkhaddressindex, saved_height)?;
        self.p2shaddressindex_to_p2shbytes
            .truncate_if_needed(p2shaddressindex, saved_height)?;
        self.p2traddressindex_to_p2trbytes
            .truncate_if_needed(p2traddressindex, saved_height)?;
        self.p2wpkhaddressindex_to_p2wpkhbytes
            .truncate_if_needed(p2wpkhaddressindex, saved_height)?;
        self.p2wshaddressindex_to_p2wshbytes
            .truncate_if_needed(p2wshaddressindex, saved_height)?;
        self.txindex_to_base_size
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_first_inputindex
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_first_outputindex
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_is_explicitly_rbf
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_rawlocktime
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_total_size
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_txid
            .truncate_if_needed(txindex, saved_height)?;
        self.txindex_to_txversion
            .truncate_if_needed(txindex, saved_height)?;
        self.unknownoutputindex_to_txindex
            .truncate_if_needed(unknownoutputindex, saved_height)?;

        Ok(())
    }

    pub fn push_bytes_if_needed(&mut self, index: TypeIndex, bytes: AddressBytes) -> Result<()> {
        match bytes {
            AddressBytes::P2PK65(bytes) => self
                .p2pk65addressindex_to_p2pk65bytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2PK33(bytes) => self
                .p2pk33addressindex_to_p2pk33bytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2PKH(bytes) => self
                .p2pkhaddressindex_to_p2pkhbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2SH(bytes) => self
                .p2shaddressindex_to_p2shbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2WPKH(bytes) => self
                .p2wpkhaddressindex_to_p2wpkhbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2WSH(bytes) => self
                .p2wshaddressindex_to_p2wshbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2TR(bytes) => self
                .p2traddressindex_to_p2trbytes
                .push_if_needed(index.into(), bytes),
            AddressBytes::P2A(bytes) => self
                .p2aaddressindex_to_p2abytes
                .push_if_needed(index.into(), bytes),
        }
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.mut_vecs()
            .into_par_iter()
            .try_for_each(|vec| vec.flush(height))
    }

    pub fn starting_height(&mut self) -> Height {
        self.mut_vecs()
            .into_iter()
            .map(|vec| {
                let h = vec.height();
                if h > Height::ZERO { h.incremented() } else { h }
            })
            .min()
            .unwrap()
    }

    pub fn vecs(&self) -> Vec<&dyn AnyCollectableVec> {
        vec![
            &self.emptyoutputindex_to_txindex,
            &self.height_to_blockhash,
            &self.height_to_difficulty,
            &self.height_to_first_emptyoutputindex,
            &self.height_to_first_inputindex,
            &self.height_to_first_opreturnindex,
            &self.height_to_first_outputindex,
            &self.height_to_first_p2aaddressindex,
            &self.height_to_first_p2msoutputindex,
            &self.height_to_first_p2pk33addressindex,
            &self.height_to_first_p2pk65addressindex,
            &self.height_to_first_p2pkhaddressindex,
            &self.height_to_first_p2shaddressindex,
            &self.height_to_first_p2traddressindex,
            &self.height_to_first_p2wpkhaddressindex,
            &self.height_to_first_p2wshaddressindex,
            &self.height_to_first_txindex,
            &self.height_to_first_unknownoutputindex,
            &self.height_to_timestamp,
            &self.height_to_total_size,
            &self.height_to_weight,
            &self.inputindex_to_outputindex,
            &self.opreturnindex_to_txindex,
            &self.outputindex_to_outputtype,
            &self.outputindex_to_typeindex,
            &self.outputindex_to_value,
            &self.p2aaddressindex_to_p2abytes,
            &self.p2msoutputindex_to_txindex,
            &self.p2pk33addressindex_to_p2pk33bytes,
            &self.p2pk65addressindex_to_p2pk65bytes,
            &self.p2pkhaddressindex_to_p2pkhbytes,
            &self.p2shaddressindex_to_p2shbytes,
            &self.p2traddressindex_to_p2trbytes,
            &self.p2wpkhaddressindex_to_p2wpkhbytes,
            &self.p2wshaddressindex_to_p2wshbytes,
            &self.txindex_to_base_size,
            &self.txindex_to_first_inputindex,
            &self.txindex_to_first_outputindex,
            &self.txindex_to_is_explicitly_rbf,
            &self.txindex_to_rawlocktime,
            &self.txindex_to_total_size,
            &self.txindex_to_txid,
            &self.txindex_to_txversion,
            &self.unknownoutputindex_to_txindex,
        ]
    }

    fn mut_vecs(&mut self) -> Vec<&mut dyn AnyIndexedVec> {
        vec![
            &mut self.emptyoutputindex_to_txindex,
            &mut self.height_to_blockhash,
            &mut self.height_to_difficulty,
            &mut self.height_to_first_emptyoutputindex,
            &mut self.height_to_first_inputindex,
            &mut self.height_to_first_opreturnindex,
            &mut self.height_to_first_outputindex,
            &mut self.height_to_first_p2aaddressindex,
            &mut self.height_to_first_p2msoutputindex,
            &mut self.height_to_first_p2pk33addressindex,
            &mut self.height_to_first_p2pk65addressindex,
            &mut self.height_to_first_p2pkhaddressindex,
            &mut self.height_to_first_p2shaddressindex,
            &mut self.height_to_first_p2traddressindex,
            &mut self.height_to_first_p2wpkhaddressindex,
            &mut self.height_to_first_p2wshaddressindex,
            &mut self.height_to_first_txindex,
            &mut self.height_to_first_unknownoutputindex,
            &mut self.height_to_timestamp,
            &mut self.height_to_total_size,
            &mut self.height_to_weight,
            &mut self.inputindex_to_outputindex,
            &mut self.opreturnindex_to_txindex,
            &mut self.outputindex_to_outputtype,
            &mut self.outputindex_to_typeindex,
            &mut self.outputindex_to_value,
            &mut self.p2aaddressindex_to_p2abytes,
            &mut self.p2msoutputindex_to_txindex,
            &mut self.p2pk33addressindex_to_p2pk33bytes,
            &mut self.p2pk65addressindex_to_p2pk65bytes,
            &mut self.p2pkhaddressindex_to_p2pkhbytes,
            &mut self.p2shaddressindex_to_p2shbytes,
            &mut self.p2traddressindex_to_p2trbytes,
            &mut self.p2wpkhaddressindex_to_p2wpkhbytes,
            &mut self.p2wshaddressindex_to_p2wshbytes,
            &mut self.txindex_to_base_size,
            &mut self.txindex_to_first_inputindex,
            &mut self.txindex_to_first_outputindex,
            &mut self.txindex_to_is_explicitly_rbf,
            &mut self.txindex_to_rawlocktime,
            &mut self.txindex_to_total_size,
            &mut self.txindex_to_txid,
            &mut self.txindex_to_txversion,
            &mut self.unknownoutputindex_to_txindex,
        ]
    }
}
