use bitcoin::address::{Address as BAddress, Payload as BPayload};
use bitcoin::blockdata::locktime::absolute::{Height as BHeight, LockTime as BLockTime};
use bitcoin::blockdata::script::ScriptBuf as BScriptBuf;
use bitcoin::blockdata::transaction::OutPoint as BOutPoint;
use bitcoin::consensus::{Decodable, Encodable};
use bitcoin::hashes::Hash as BHashTrait;
use bitcoin::hashes::hash160::Hash as BHash;
use bitcoin::hash_types::PubkeyHash as BPubkeyHash;
use bitcoin::script::PushBytesBuf as BPushBytesBuf;
use bitcoin::sighash::{LegacySighash as BLegacySighash, SighashCache as BSighashCache};
use bitcoin::transaction::Transaction as BTransaction;
use bitcoin::{Sequence as BSequence, TxIn as BTxIn, TxOut as BTxOut, Witness as BWitness};
use bitcoin::opcodes::All as AnyOpcode;
use claim::TransactionSigner;
use std::str::FromStr;
use tw_hash::H256;
use tw_keypair::ecdsa::secp256k1;
use tw_keypair::traits::KeyPairTrait;

pub mod claim;

pub type Result<T> = std::result::Result<T, Error>;

const SIGHASH_ALL: u32 = 1;

pub enum SigHashType {
    All,
    None,
    Single,
    AnyoneCanPay,
}

impl SigHashType {
    fn to_le_byte(&self) -> u8 {
        match self {
            SigHashType::All => 1_u8.to_le(),
            SigHashType::None => 2_u8.to_le(),
            SigHashType::Single => 3_u8.to_le(),
            // 0x80 = 128
            SigHashType::AnyoneCanPay => 128_u8.to_le(),
        }
    }
}

pub fn keypair_from_wif(wif: &str) -> Result<secp256k1::KeyPair> {
    secp256k1::KeyPair::from_wif(wif).map_err(|_| Error::Todo)
}

fn convert_legacy_btc_hash_to_h256(hash: BLegacySighash) -> H256 {
    let slice: &[u8] = hash.as_raw_hash().as_ref();
    debug_assert_eq!(slice.len(), 32);
    let bytes: [u8; 32] = slice.try_into().unwrap();
    H256::from(bytes)
}

#[derive(Debug, Clone)]
pub enum Error {
    Todo,
}

/*
#[test]
fn poc() {
    let secp = Secp256k1::new();
    let (private, public) = generate_keypair(&mut rand::thread_rng());
    let tweaked = UntweakedPublicKey::from(public);

    let script1 = BScriptBuf::new();
    let script2 = BScriptBuf::new();

    let spend_info = TaprootBuilder::new()
        .add_leaf(1, script1.clone())
        .unwrap()
        .add_leaf(1, script2)
        .unwrap()
        .finalize(&secp, tweaked)
        .unwrap();

    let root = spend_info.merkle_root().unwrap();

    let tapscript = BScriptBuf::new_v1_p2tr(&secp, tweaked, Some(root));

    let control_block = spend_info.control_block(&(script1, LeafVersion::TapScript));
}
*/

pub struct ScriptHash;

#[derive(Debug, Clone)]
pub struct TransactionBuilder {
    version: i32,
    lock_time: BLockTime,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    btc_tx: Option<BTransaction>,
}

impl Default for TransactionBuilder {
    fn default() -> Self {
        TransactionBuilder {
            // TODO: Check this.
            version: 2,
            // No lock time, transaction is immediately spendable.
            lock_time: BLockTime::Blocks(BHeight::ZERO),
            inputs: vec![],
            outputs: vec![],
            btc_tx: None,
        }
    }
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn version(mut self, version: i32) -> Self {
        self.version = version;
        self
    }
    // TODO: handle locktime seconds?.
    pub fn lock_time(mut self, height: u32) -> Self {
        self.lock_time = BLockTime::Blocks(BHeight::from_consensus(height).unwrap());
        self
    }
    pub fn add_input(mut self, input: TxInput) -> Self {
        self.inputs.push(input);
        self
    }
    pub fn add_output(mut self, output: TxOutput) -> Self {
        self.outputs.push(output);
        self
    }
    pub fn prepare_for_signing(mut self) -> Self {
        // Prepare boilerplate transaction for `bitcoin` crate.
        let mut tx = BTransaction {
            version: self.version,
            lock_time: self.lock_time,
            input: vec![],
            output: vec![],
        };

        // Prepare the inputs for `bitcoin` crate.
        for input in self.inputs.iter().cloned() {
            let btc_txin = match input {
                // TODO: `BTxIn` should implement `From<TxInput>`.
                TxInput::P2PKH(p2pkh) => BTxIn::from(p2pkh.ctx),
                TxInput::NonStandard { ctx } => BTxIn::from(ctx),
            };

            tx.input.push(btc_txin);
        }

        // Prepare the outputs for `bitcoin` crate.
        for output in &self.outputs {
            // TODO: Doable without clone?
            let btc_txout = BTxOut::from(output.clone());
            tx.output.push(btc_txout);
        }

        self.btc_tx = Some(tx);
        self
    }
    pub fn sign_inputs<S>(self, signer: S) -> Result<Self>
    where
        S: TransactionSigner,
    {
        self.sign_inputs_fn(|input, sighash| match input {
            TxInput::P2PKH(p2pkh) => signer
                .claim_p2pkh(p2pkh, sighash)
                // TODO: Should not convert into BScriptBuf here.
                .map(|claim| claim.into_script()),
            TxInput::NonStandard { ctx: _ } => {
                panic!()
            },
        })
    }
    // TODO: Does this have to return `Result<T>`?
    pub fn sign_inputs_fn<F>(mut self, signer: F) -> Result<Self>
    where
        F: Fn(&TxInput, H256) -> Result<BScriptBuf>,
    {
        let cache = BSighashCache::new(self.btc_tx.unwrap());

        let mut updated_scriptsigs = vec![];

        // For each input (index), we create a hash which is to be signed.
        for (index, input) in self.inputs.iter().enumerate() {
            match input {
                TxInput::P2PKH(p2pkh) => {
                    let legacy_hash = cache
                        .legacy_signature_hash(
                            index,
                            // TODO: Add note that this is same as `scriptPubKey`,
                            // handled somewhere else.
                            &p2pkh.ctx.script_pubkey,
                            // TODO: Make adjustable.
                            SIGHASH_ALL,
                        )
                        .map_err(|_| Error::Todo)?;

                    let h256 = convert_legacy_btc_hash_to_h256(legacy_hash);
                    // TODO: Rename closure var.
                    let updated = signer(input, h256)?;

                    updated_scriptsigs.push((index, updated));
                },
                // Skip.
                TxInput::NonStandard { ctx: _ } => continue,
            };
        }

        let mut tx = cache.into_transaction();

        // Update the transaction with the updated scriptSig's.
        for (index, script_sig) in updated_scriptsigs {
            tx.input[index].script_sig = script_sig;
        }

        self.btc_tx = Some(tx);

        // TODO: Return new type.
        Ok(self)
    }
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![];
        self.btc_tx
            .as_ref()
            .unwrap()
            .consensus_encode(&mut buffer)
            .map_err(|_| Error::Todo)?;
        Ok(buffer)
    }
}

pub enum TransactionSigHashType {
    Legacy(H256),
}

impl TransactionSigHashType {
    pub fn as_h256(&self) -> &H256 {
        match self {
            TransactionSigHashType::Legacy(h256) => h256,
        }
    }
}

#[derive(Debug, Clone)]
// TODO: Should be private.
pub struct InputContext {
    pub previous_output: BOutPoint,
    // The condition for claiming the output.
    pub script_pubkey: BScriptBuf,
    // TODO: Document this.
    pub sequence: BSequence,
    // BWitness data for Segwit/Taproot transactions.
    pub witness: BWitness,
}

impl InputContext {
    pub fn new(utxo: BTxOut, point: BOutPoint) -> Self {
        InputContext {
            previous_output: point,
            // TODO: Document this.
            script_pubkey: utxo.script_pubkey,
            // Default value of `0xFFFFFFFF = 4294967295`.
            sequence: BSequence::default(),
            // Empty witness.
            witness: BWitness::new(),
        }
    }
    pub fn from_slice(mut slice: &[u8]) -> Result<Self> {
        Ok(InputContext {
            previous_output: Decodable::consensus_decode_from_finite_reader(&mut slice)
                .map_err(|_| Error::Todo)?,
            script_pubkey: Decodable::consensus_decode_from_finite_reader(&mut slice)
                .map_err(|_| Error::Todo)?,
            sequence: Decodable::consensus_decode_from_finite_reader(&mut slice)
                .map_err(|_| Error::Todo)?,
            witness: BWitness::default(),
        })
    }
}

impl From<InputContext> for BTxIn {
    fn from(ctx: InputContext) -> Self {
        BTxIn {
            previous_output: ctx.previous_output,
            // TODO: Document this.
            script_sig: BScriptBuf::default(),
            sequence: ctx.sequence,
            witness: ctx.witness,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TxOutput {
    P2PKH(TxOutputP2PKH),
}

impl From<TxOutputP2PKH> for TxOutput {
    fn from(output: TxOutputP2PKH) -> Self {
        TxOutput::P2PKH(output)
    }
}

#[derive(Debug, Clone)]
pub struct TxOutputP2PKH {
    satoshis: u64,
    script_pubkey: BScriptBuf,
}

impl TxOutputP2PKH {
    pub fn new(satoshis: u64, recipient: &PubkeyHash) -> Self {
        TxOutputP2PKH {
            satoshis,
            script_pubkey: BScriptBuf::new_p2pkh(&recipient.0),
        }
    }
}

impl From<TxOutput> for BTxOut {
    fn from(out: TxOutput) -> Self {
        match out {
            TxOutput::P2PKH(p2pkh) => BTxOut {
                value: p2pkh.satoshis,
                script_pubkey: p2pkh.script_pubkey,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum TxInput {
    P2PKH(TxInputP2PKH),
    NonStandard { ctx: InputContext },
}

#[derive(Debug, Clone)]
pub struct TxInputP2PKH {
    pub ctx: InputContext,
    pub recipient: PubkeyHash,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PubkeyHash(BPubkeyHash);

impl PubkeyHash {
    pub fn from_address_str(slice: &str) -> Result<Self> {
        let checked = BAddress::from_str(slice).map_err(|_| Error::Todo)?;

        // TODO: Network should be checked.
        let hash = match checked.payload {
            BPayload::PubkeyHash(hash) => hash,
            _ => todo!(),
        };

        Ok(PubkeyHash(hash))
    }
    pub fn from_keypair(keypair: &secp256k1::KeyPair, compressed: bool) -> Result<Self> {
        let bhash = if compressed {
            BHash::hash(keypair.public().compressed().as_slice())
        } else {
            BHash::hash(keypair.public().uncompressed().as_slice())
        };

        let pubkey = BPubkeyHash::from_raw_hash(bhash);

        Ok(PubkeyHash(pubkey))
    }
    pub fn from_bytes(bytes: [u8; 20]) -> Result<Self> {
        Ok(PubkeyHash(BPubkeyHash::from_byte_array(bytes)))
    }
    pub fn from_script(script: &BScriptBuf) -> Result<Self> {
        let pubkey = match BPayload::from_script(script).map_err(|_| Error::Todo)? {
            BPayload::PubkeyHash(hash) => PubkeyHash(hash),
            _ => return Err(Error::Todo),
        };

        Ok(pubkey)
    }
}

impl TxInput {
    pub fn new_p2pkh() -> Self {
        todo!()
    }
    pub fn from_slice(slice: &[u8]) -> Result<Self> {
        let ctx = InputContext::from_slice(slice)?;
        let recipient = PubkeyHash::from_script(&ctx.script_pubkey)?;

        Ok(TxInput::P2PKH(TxInputP2PKH { ctx, recipient }))
    }
}

fn get_push(size: u32) -> Result<(AnyOpcode, Option<Vec<u8>>)> {
    use bitcoin::opcodes::all::*;

    let ret = match size {
        // OP_PUSHBYTES[0|1|2|...|75]
        0..=75 => (bitcoin::opcodes::All::from(size as u8), None),
        76..=255 => (OP_PUSHDATA1, Some(size.to_le_bytes().to_vec())),
        256..=65535 => (OP_PUSHDATA2, Some(size.to_le_bytes().to_vec())),
        65536..=u32::MAX => (OP_PUSHDATA4, Some(size.to_le_bytes().to_vec())),
        _ => return Err(Error::Todo)
    };

    Ok(ret)
}

fn create_envelope(content_type: &str, data: &str) -> Result<BScriptBuf> {
    use bitcoin::opcodes::*;
    use bitcoin::opcodes::all::*;

    // TODO: Check overflow
    // Prepare content-type buffer.
    let mut content_ty_buf = BPushBytesBuf::new();
    let (op_push, size_buf) = get_push(content_type.len() as u32)?;

    // For any sizes below 75, we use encode as `OP_PUSHBYTES[0|1|2|...|75]`.
    // Fany any sized above 75, we use encode as `OP_PUSHDATA[1|2|3|4] <SIZE_BUF>`.
    content_ty_buf.push(op_push.to_u8()).unwrap();
    if let Some(size_buf) = size_buf {
        content_ty_buf.extend_from_slice(size_buf.as_slice()).unwrap();
    }

    // Prepare data buffer.
    let mut data_buf = BPushBytesBuf::new();

    let x = BScriptBuf::builder()
        .push_opcode(OP_FALSE)
        .push_opcode(OP_IF)
        // Push three bytes of "orb"
        .push_opcode(OP_PUSHBYTES_3)
        .push_slice(BPushBytesBuf::try_from(b"orb").unwrap())
        // OP_TRUE = OP_1
        .push_opcode(OP_TRUE)
        .push_slice(content_ty_buf)
        .push_opcode(OP_0)
        .push_slice(data_buf)
        .push_opcode(OP_ENDIF);

    todo!()
}
