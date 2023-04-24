type Usize = u64;

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BitVec<Block> {
    bits: Option<Block>,
    len: u64,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Bloom<T> {
    pub keys: Vec<u64>,
    pub bits: BitVec<u64>,
    num_bits_set: u64,
    //_phantom: PhantomData<T>,
    _phantom: Option<T>,
}

type Pubkey = [u8; 32];

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hash(pub [u8; 32]);

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Signature(pub [u8; 32]);

// Transaction
#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Message {
    pub header: MessageHeader,
    pub account_keys: Vec<Pubkey>,
    pub recent_blockhash: Hash,
    pub instructions: Vec<CompiledInstruction>,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Transaction {
    pub signatures: Vec<Signature>,
    pub message: Message,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SocketAddr {
    ip: [u8; 4],
    port: u16,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegacyContactInfo {
    pub id: Pubkey,
    pub gossip: SocketAddr,
    pub tvu: SocketAddr,
    pub tvu_forwards: SocketAddr,
    pub repair: SocketAddr,
    pub tpu: SocketAddr,
    pub tpu_forwards: SocketAddr,
    pub tpu_vote: SocketAddr,
    pub rpc: SocketAddr,
    pub rpc_pubsub: SocketAddr,
    pub serve_repair: SocketAddr,
    pub wallclock: u64,
    pub shred_version: u16,
}

fn socketaddr_default() -> SocketAddr {
    SocketAddr {
        ip: [0; 4],
        port: 0,
    }
}

impl Default for LegacyContactInfo {
    fn default() -> Self {
        LegacyContactInfo {
            id: Pubkey::default(),
            gossip: socketaddr_default(),
            tvu: socketaddr_default(),
            tvu_forwards: socketaddr_default(),
            repair: socketaddr_default(),
            tpu: socketaddr_default(),
            tpu_forwards: socketaddr_default(),
            tpu_vote: socketaddr_default(),
            rpc: socketaddr_default(),
            rpc_pubsub: socketaddr_default(),
            serve_repair: socketaddr_default(),
            wallclock: 0,
            shred_version: 0,
        }
    }
}

pub type VoteIndex = u8;

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vote {
    pub from: Pubkey,
    transaction: Transaction,
    pub wallclock: u64,
}

pub type Slot = u64;

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SnapshotHashes {
    pub from: Pubkey,
    pub hashes: Vec<(Slot, Hash)>,
    pub wallclock: u64,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegacyVersion1 {
    major: u16,
    minor: u16,
    patch: u16,
    commit: Option<u32>, // first 4 bytes of the sha1 commit hash
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegacyVersion {
    pub from: Pubkey,
    pub wallclock: u64,
    pub version: LegacyVersion1,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegacyVersion2 {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
    pub commit: Option<u32>,
    pub feature_set: u32,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Version {
    pub from: Pubkey,
    pub wallclock: u64,
    pub version: LegacyVersion2,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeInstance {
    pub from: Pubkey,
    pub wallclock: u64,
    pub timestamp: u64,
    pub token: u64,
}

pub type EpochSlotsIndex = u8;

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Flate2 {
    pub first_slot: Slot,
    pub num: Usize,
    pub compressed: Vec<u8>,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Uncompressed {
    pub first_slot: Slot,
    pub num: Usize,
    pub slots: BitVec<u8>,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompressedSlots {
    Flate2(Flate2),
    Uncompressed(Uncompressed),
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EpochSlots {
    pub from: Pubkey,
    pub slots: Vec<CompressedSlots>,
    pub wallclock: u64,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(dead_code)]
pub enum DeprecatedCompressionType {
    Uncompressed,
    GZip,
    BZip2,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedEpochIncompleteSlots {
    pub first: Slot,
    pub compression: DeprecatedCompressionType,
    pub compressed_list: Vec<u8>,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LowestSlot {
    pub from: Pubkey,
    pub root: Slot,
    pub lowest: Slot,
    pub slots: Vec<Slot>,
    pub stash: Vec<DeprecatedEpochIncompleteSlots>,
    pub wallclock: u64,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IncrementalSnapshotHashes {
    pub from: Pubkey,
    pub base: (Slot, Hash),
    pub hashes: Vec<(Slot, Hash)>,
    pub wallclock: u64,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CrdsData {
    LegacyContactInfo(LegacyContactInfo),    // OK len:254
    Vote(VoteIndex, Vote),                   // OK len:472
    LowestSlot(u8, LowestSlot),              // OK len:185
    SnapshotHashes(SnapshotHashes),          // OK len:240
    AccountsHashes(SnapshotHashes),          // OK len:800
    EpochSlots(EpochSlotsIndex, EpochSlots), // OK len:1049
    LegacyVersion(LegacyVersion),            // OK len:163
    Version(Version),                        // OK len:167
    NodeInstance(NodeInstance),              // OK len:168
    DuplicateShred(),                        // ??
    IncrementalSnapshotHashes(IncrementalSnapshotHashes), // OK len:360
    ContactInfo(),                           // ??
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CrdsValue {
    pub signature: Signature,
    pub data: CrdsData,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CrdsFilter {
    pub filter: Bloom<Hash>,
    pub mask: u64,
    pub mask_bits: u32,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PingGeneric<T> {
    pub from: Pubkey,
    pub token: T,
    pub signature: Signature,
}

pub type Ping = PingGeneric<[u8; 32]>;

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pong {
    pub from: Pubkey,
    pub hash: Hash,
    pub signature: Signature,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "alkahest",
    derive(alkahest::Formula, alkahest::Serialize, alkahest::Deserialize)
)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)
)]
#[cfg_attr(feature = "rkyv", archive_attr(derive(bytecheck::CheckBytes)))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::large_enum_variant)]
pub enum Protocol {
    PullRequest(CrdsFilter, CrdsValue),
    PullResponse(Pubkey, Vec<CrdsValue>),
    PushMessage(Pubkey, Vec<CrdsValue>),
    PruneMessage(Pubkey),
    PingMessage(Ping),
    PongMessage(Pong),
}

pub fn crate_ping() -> Protocol {
    Protocol::PingMessage(Ping {
        from: rand::random(),
        token: rand::random(),
        signature: Signature(rand::random()),
    })
}

pub fn crate_pong() -> Protocol {
    Protocol::PongMessage(Pong {
        from: rand::random(),
        hash: Hash(rand::random()),
        signature: Signature(rand::random()),
    })
}

pub fn crate_pull_request_legacy_contact_info() -> Protocol {
    Protocol::PullRequest(
        CrdsFilter {
            filter: Bloom {
                keys: (1..10)
                    .map(|_| rand::random())
                    .collect::<Vec<_>>(),
                bits: BitVec {
                    bits: None,
                    len: rand::random(),
                },
                num_bits_set: rand::random(),
                _phantom: Some(Hash(rand::random())),
            },
            mask: rand::random(),
            mask_bits: rand::random(),
        },
        CrdsValue {
            signature: Signature(rand::random()),
            data: CrdsData::LegacyContactInfo(LegacyContactInfo::default()),
        },
    )
}

pub fn crate_pull_response_legacy_contact_info() -> Protocol {
    Protocol::PullResponse(
        rand::random(),
        vec![
            CrdsValue {
                signature: Signature(rand::random()),
                data: CrdsData::LegacyContactInfo(LegacyContactInfo::default()),
            },
            CrdsValue {
                signature: Signature(rand::random()),
                data: CrdsData::Vote(
                    rand::random(),
                    Vote {
                        from: rand::random(),
                        transaction: Transaction {
                            signatures: (1..10)
                                .map(|_| Signature(rand::random()))
                                .collect::<Vec<_>>(),
                            message: Message {
                                header: MessageHeader {
                                    num_required_signatures: rand::random(),
                                    num_readonly_signed_accounts: rand::random(),
                                    num_readonly_unsigned_accounts: rand::random(),
                                },
                                account_keys: (1..10)
                                    .map(|_| rand::random())
                                    .collect::<Vec<_>>(),
                                recent_blockhash: Hash(rand::random()),
                                instructions: (1..10)
                                    .map(|_| CompiledInstruction {
                                        program_id_index: rand::random(),
                                        accounts: (1..10)
                                            .map(|_| rand::random())
                                            .collect::<Vec<_>>(),
                                        data: (1..10)
                                            .map(|_| rand::random())
                                            .collect::<Vec<_>>(),
                                    })
                                    .collect::<Vec<_>>(),
                            },
                        },
                        wallclock: rand::random(),
                    },
                ),
            },
            CrdsValue {
                signature: Signature(rand::random()),
                data: CrdsData::LowestSlot(
                    rand::random(),
                    LowestSlot {
                        from: rand::random(),
                        root: rand::random(),
                        lowest: rand::random(),
                        slots: (1..10)
                            .map(|_| rand::random())
                            .collect::<Vec<_>>(),
                        stash: (1..10)
                            .map(|_| DeprecatedEpochIncompleteSlots {
                                first: rand::random(),
                                compression: DeprecatedCompressionType::Uncompressed,
                                compressed_list: (1..10)
                                    .map(|_| rand::random())
                                    .collect::<Vec<_>>(),
                            })
                            .collect::<Vec<_>>(),
                        wallclock: rand::random(),
                    },
                ),
            },
            CrdsValue {
                signature: Signature(rand::random()),
                data: CrdsData::SnapshotHashes(SnapshotHashes {
                    from: rand::random(),
                    hashes: (1..10)
                        .map(|_| (8_u64, Hash(rand::random())))
                        .collect::<Vec<(u64, Hash)>>(),
                    wallclock: rand::random(),
                }),
            },
        ],
    )
}
