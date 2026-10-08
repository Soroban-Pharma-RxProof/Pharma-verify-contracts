import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CA2JMBWAT2DDZZHCULUJXWBZ7N27QO2LRADSPNR4CDUULBWIGCJ2CAZO",
  }
} as const


/**
 * Batch details stored in persistent storage
 */
export interface Batch {
  batch_id: Buffer;
  created_at: u64;
  current_custody: string;
  dispensed_packs: u32;
  expiry_timestamp: u64;
  manufacturer: string;
  merkle_root: Buffer;
  metadata_hash: Buffer;
  packaging_spec: PackagingSpec;
  recall_reason: Option<string>;
  status: BatchStatus;
  total_quantity: u32;
}

/**
 * Contract error definitions
 */
export const Errors = {
  1: {message:"AlreadyInitialized"},

  2: {message:"NotInitialized"},

  3: {message:"Unauthorized"},

  4: {message:"ContractPaused"},

  5: {message:"ParticipantNotFound"},

  6: {message:"ParticipantInactive"},

  7: {message:"ParticipantAlreadyExists"},

  8: {message:"InvalidRole"},

  9: {message:"BatchAlreadyExists"},

  10: {message:"BatchNotFound"},

  11: {message:"BatchExpired"},

  12: {message:"BatchRecalled"},

  13: {message:"InvalidMerkleProof"},

  14: {message:"ProofDepthExceeded"},

  15: {message:"SerialAlreadyDispensed"},

  16: {message:"InvalidStripIndex"},

  17: {message:"StripAlreadyDispensed"},

  18: {message:"BatchQuantityExceeded"},

  19: {message:"NotCurrentCustodian"},

  20: {message:"RegulatorAlreadyExists"},

  21: {message:"RegulatorNotFound"},

  22: {message:"ProposalNotFound"},

  23: {message:"AlreadyVoted"},

  24: {message:"ProposalAlreadyExecuted"},

  25: {message:"CannotRemoveLastRegulator"},

  26: {message:"InvalidParameters"}
}
/**
 * Contract storage keys
 */
export type DataKey = {tag: "Admin", values: void} | {tag: "IsPaused", values: void} | {tag: "Regulator", values: readonly [string]} | {tag: "RegulatorCount", values: void} | {tag: "ProposalCounter", values: void} | {tag: "RegulatorProposal", values: readonly [u64]} | {tag: "ProposalVoted", values: readonly [u64, string]} | {tag: "Participant", values: readonly [string]} | {tag: "Batch", values: readonly [Buffer]} | {tag: "SerialDispensed", values: readonly [Buffer, Buffer]} | {tag: "DispensedStrips", values: readonly [Buffer, Buffer]} | {tag: "SuspiciousReported", values: readonly [Buffer, Buffer]};

/**
 * Life-cycle status of a manufactured batch
 */
export enum BatchStatus {
  Active = 1,
  Recalled = 2,
}


/**
 * Participant registration record on-chain
 */
export interface Participant {
  active: boolean;
  address: string;
  metadata_hash: Buffer;
  registered_at: u64;
  role: ParticipantRole;
}


/**
 * Packaging specifications for blister packs or bulk packages
 */
export interface PackagingSpec {
  /**
 * Total blister strips per pack (default 1 for bottle/box)
 */
total_strips: u32;
  /**
 * Number of individual dosage units per strip
 */
units_per_strip: u32;
}

/**
 * Role assigned to authorized supply chain participants
 */
export enum ParticipantRole {
  Manufacturer = 1,
  Distributor = 2,
  Pharmacy = 3,
  Regulator = 4,
}


/**
 * Multi-signature proposal record for removing a regulator
 */
export interface RegulatorProposal {
  approvals_count: u32;
  created_at: u64;
  executed: boolean;
  proposal_id: u64;
  proposer: string;
  target_regulator: string;
}

/**
 * Verification verdict for a pack query
 */
export enum VerificationResult {
  Authentic = 1,
  Expired = 2,
  Recalled = 3,
  Suspicious = 4,
  Invalid = 5,
}


export interface Client {
  /**
   * Construct and simulate a pause transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Emergency pause (halts state mutations during security incidents)
   */
  pause: ({caller}: {caller: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a unpause transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Emergency unpause
   */
  unpause: ({caller}: {caller: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_admin transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_admin: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<string>>>

  /**
   * Construct and simulate a get_batch transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_batch: ({batch_id}: {batch_id: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<Batch>>>

  /**
   * Construct and simulate a is_paused transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_paused: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a initialize transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Initialize contract with an administrator and default regulator role
   */
  initialize: ({admin}: {admin: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_custody transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_custody: ({batch_id}: {batch_id: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<string>>>

  /**
   * Construct and simulate a verify_pack transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Verify a pack's authenticity via Merkle proof and check expiry, recalls, and clone status
   */
  verify_pack: ({batch_id, serial_hash, merkle_proof, strip_index}: {batch_id: Buffer, serial_hash: Buffer, merkle_proof: Array<Buffer>, strip_index: Option<u32>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<VerificationResult>>

  /**
   * Construct and simulate a get_proposal transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_proposal: ({proposal_id}: {proposal_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<RegulatorProposal>>>

  /**
   * Construct and simulate a is_regulator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_regulator: ({regulator}: {regulator: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a recall_batch transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Issue a batch recall (authorized manufacturer or regulators)
   */
  recall_batch: ({caller, batch_id, reason}: {caller: string, batch_id: Buffer, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a add_regulator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Add an authorized regulator (admin or existing regulator only)
   */
  add_regulator: ({caller, new_regulator}: {caller: string, new_regulator: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a dispense_pack transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Dispense a pack or blister strip at an authorized pharmacy, burning the code
   */
  dispense_pack: ({caller, batch_id, serial_hash, merkle_proof, strip_index}: {caller: string, batch_id: Buffer, serial_hash: Buffer, merkle_proof: Array<Buffer>, strip_index: Option<u32>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a register_batch transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Register a new medicine batch with Merkle root and packaging specifications
   */
  register_batch: ({caller, batch_id, merkle_root, total_quantity, packaging_spec, expiry_timestamp, metadata_hash}: {caller: string, batch_id: Buffer, merkle_root: Buffer, total_quantity: u32, packaging_spec: PackagingSpec, expiry_timestamp: u64, metadata_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_participant transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_participant: ({participant}: {participant: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<Participant>>>

  /**
   * Construct and simulate a transfer_custody transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Transfer batch custody along the supply chain
   */
  transfer_custody: ({caller, batch_id, to}: {caller: string, batch_id: Buffer, to: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a report_suspicious transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Report a suspicious or cloned pack (open to public, patients, and inspectors)
   */
  report_suspicious: ({caller, batch_id, serial_hash, reason}: {caller: string, batch_id: Buffer, serial_hash: Buffer, reason: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a get_regulator_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_regulator_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a is_serial_dispensed transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_serial_dispensed: ({batch_id, serial_hash}: {batch_id: Buffer, serial_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a get_dispensed_strips transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  get_dispensed_strips: ({batch_id, serial_hash}: {batch_id: Buffer, serial_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a register_participant transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Register a supply chain participant (regulator only)
   */
  register_participant: ({caller, participant, role, metadata_hash}: {caller: string, participant: string, role: ParticipantRole, metadata_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a is_suspicious_reported transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   */
  is_suspicious_reported: ({batch_id, serial_hash}: {batch_id: Buffer, serial_hash: Buffer}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<boolean>>

  /**
   * Construct and simulate a set_participant_active transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Suspend or activate a participant (regulator only)
   */
  set_participant_active: ({caller, participant, active}: {caller: string, participant: string, active: boolean}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a approve_remove_regulator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Approve an active regulator removal proposal (multi-signature threshold)
   */
  approve_remove_regulator: ({caller, proposal_id}: {caller: string, proposal_id: u64}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<void>>>

  /**
   * Construct and simulate a propose_remove_regulator transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Propose removal of a regulator under multi-signature governance
   */
  propose_remove_regulator: ({caller, regulator_to_remove}: {caller: string, regulator_to_remove: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Result<u64>>>

}
export class Client extends ContractClient {
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAAEFFbWVyZ2VuY3kgcGF1c2UgKGhhbHRzIHN0YXRlIG11dGF0aW9ucyBkdXJpbmcgc2VjdXJpdHkgaW5jaWRlbnRzKQAAAAAAAAVwYXVzZQAAAAAAAAEAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAABFFbWVyZ2VuY3kgdW5wYXVzZQAAAAAAAAd1bnBhdXNlAAAAAAEAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAPoAAAAEw==",
        "AAAAAAAAAAAAAAAJZ2V0X2JhdGNoAAAAAAAAAQAAAAAAAAAIYmF0Y2hfaWQAAAPuAAAAIAAAAAEAAAPoAAAH0AAAAAVCYXRjaAAAAA==",
        "AAAAAAAAAAAAAAAJaXNfcGF1c2VkAAAAAAAAAAAAAAEAAAAB",
        "AAAAAAAAAERJbml0aWFsaXplIGNvbnRyYWN0IHdpdGggYW4gYWRtaW5pc3RyYXRvciBhbmQgZGVmYXVsdCByZWd1bGF0b3Igcm9sZQAAAAppbml0aWFsaXplAAAAAAABAAAAAAAAAAVhZG1pbgAAAAAAABMAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAALZ2V0X2N1c3RvZHkAAAAAAQAAAAAAAAAIYmF0Y2hfaWQAAAPuAAAAIAAAAAEAAAPoAAAAEw==",
        "AAAAAAAAAFlWZXJpZnkgYSBwYWNrJ3MgYXV0aGVudGljaXR5IHZpYSBNZXJrbGUgcHJvb2YgYW5kIGNoZWNrIGV4cGlyeSwgcmVjYWxscywgYW5kIGNsb25lIHN0YXR1cwAAAAAAAAt2ZXJpZnlfcGFjawAAAAAEAAAAAAAAAAhiYXRjaF9pZAAAA+4AAAAgAAAAAAAAAAtzZXJpYWxfaGFzaAAAAAPuAAAAIAAAAAAAAAAMbWVya2xlX3Byb29mAAAD6gAAA+4AAAAgAAAAAAAAAAtzdHJpcF9pbmRleAAAAAPoAAAABAAAAAEAAAfQAAAAElZlcmlmaWNhdGlvblJlc3VsdAAA",
        "AAAAAAAAAAAAAAAMZ2V0X3Byb3Bvc2FsAAAAAQAAAAAAAAALcHJvcG9zYWxfaWQAAAAABgAAAAEAAAPoAAAH0AAAABFSZWd1bGF0b3JQcm9wb3NhbAAAAA==",
        "AAAAAAAAAAAAAAAMaXNfcmVndWxhdG9yAAAAAQAAAAAAAAAJcmVndWxhdG9yAAAAAAAAEwAAAAEAAAAB",
        "AAAAAAAAADxJc3N1ZSBhIGJhdGNoIHJlY2FsbCAoYXV0aG9yaXplZCBtYW51ZmFjdHVyZXIgb3IgcmVndWxhdG9ycykAAAAMcmVjYWxsX2JhdGNoAAAAAwAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAhiYXRjaF9pZAAAA+4AAAAgAAAAAAAAAAZyZWFzb24AAAAAABEAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAD5BZGQgYW4gYXV0aG9yaXplZCByZWd1bGF0b3IgKGFkbWluIG9yIGV4aXN0aW5nIHJlZ3VsYXRvciBvbmx5KQAAAAAADWFkZF9yZWd1bGF0b3IAAAAAAAACAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAADW5ld19yZWd1bGF0b3IAAAAAAAATAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAExEaXNwZW5zZSBhIHBhY2sgb3IgYmxpc3RlciBzdHJpcCBhdCBhbiBhdXRob3JpemVkIHBoYXJtYWN5LCBidXJuaW5nIHRoZSBjb2RlAAAADWRpc3BlbnNlX3BhY2sAAAAAAAAFAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACGJhdGNoX2lkAAAD7gAAACAAAAAAAAAAC3NlcmlhbF9oYXNoAAAAA+4AAAAgAAAAAAAAAAxtZXJrbGVfcHJvb2YAAAPqAAAD7gAAACAAAAAAAAAAC3N0cmlwX2luZGV4AAAAA+gAAAAEAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAEtSZWdpc3RlciBhIG5ldyBtZWRpY2luZSBiYXRjaCB3aXRoIE1lcmtsZSByb290IGFuZCBwYWNrYWdpbmcgc3BlY2lmaWNhdGlvbnMAAAAADnJlZ2lzdGVyX2JhdGNoAAAAAAAHAAAAAAAAAAZjYWxsZXIAAAAAABMAAAAAAAAACGJhdGNoX2lkAAAD7gAAACAAAAAAAAAAC21lcmtsZV9yb290AAAAA+4AAAAgAAAAAAAAAA50b3RhbF9xdWFudGl0eQAAAAAABAAAAAAAAAAOcGFja2FnaW5nX3NwZWMAAAAAB9AAAAANUGFja2FnaW5nU3BlYwAAAAAAAAAAAAAQZXhwaXJ5X3RpbWVzdGFtcAAAAAYAAAAAAAAADW1ldGFkYXRhX2hhc2gAAAAAAAPuAAAAIAAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAAPZ2V0X3BhcnRpY2lwYW50AAAAAAEAAAAAAAAAC3BhcnRpY2lwYW50AAAAABMAAAABAAAD6AAAB9AAAAALUGFydGljaXBhbnQA",
        "AAAAAAAAAC1UcmFuc2ZlciBiYXRjaCBjdXN0b2R5IGFsb25nIHRoZSBzdXBwbHkgY2hhaW4AAAAAAAAQdHJhbnNmZXJfY3VzdG9keQAAAAMAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAIYmF0Y2hfaWQAAAPuAAAAIAAAAAAAAAACdG8AAAAAABMAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAE1SZXBvcnQgYSBzdXNwaWNpb3VzIG9yIGNsb25lZCBwYWNrIChvcGVuIHRvIHB1YmxpYywgcGF0aWVudHMsIGFuZCBpbnNwZWN0b3JzKQAAAAAAABFyZXBvcnRfc3VzcGljaW91cwAAAAAAAAQAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAAIYmF0Y2hfaWQAAAPuAAAAIAAAAAAAAAALc2VyaWFsX2hhc2gAAAAD7gAAACAAAAAAAAAABnJlYXNvbgAAAAAAEQAAAAEAAAPpAAAD7QAAAAAAAAAD",
        "AAAAAAAAAAAAAAATZ2V0X3JlZ3VsYXRvcl9jb3VudAAAAAAAAAAAAQAAAAQ=",
        "AAAAAAAAAAAAAAATaXNfc2VyaWFsX2Rpc3BlbnNlZAAAAAACAAAAAAAAAAhiYXRjaF9pZAAAA+4AAAAgAAAAAAAAAAtzZXJpYWxfaGFzaAAAAAPuAAAAIAAAAAEAAAAB",
        "AAAAAAAAAAAAAAAUZ2V0X2Rpc3BlbnNlZF9zdHJpcHMAAAACAAAAAAAAAAhiYXRjaF9pZAAAA+4AAAAgAAAAAAAAAAtzZXJpYWxfaGFzaAAAAAPuAAAAIAAAAAEAAAAE",
        "AAAAAAAAADRSZWdpc3RlciBhIHN1cHBseSBjaGFpbiBwYXJ0aWNpcGFudCAocmVndWxhdG9yIG9ubHkpAAAAFHJlZ2lzdGVyX3BhcnRpY2lwYW50AAAABAAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAtwYXJ0aWNpcGFudAAAAAATAAAAAAAAAARyb2xlAAAH0AAAAA9QYXJ0aWNpcGFudFJvbGUAAAAAAAAAAA1tZXRhZGF0YV9oYXNoAAAAAAAD7gAAACAAAAABAAAD6QAAA+0AAAAAAAAAAw==",
        "AAAAAAAAAAAAAAAWaXNfc3VzcGljaW91c19yZXBvcnRlZAAAAAAAAgAAAAAAAAAIYmF0Y2hfaWQAAAPuAAAAIAAAAAAAAAALc2VyaWFsX2hhc2gAAAAD7gAAACAAAAABAAAAAQ==",
        "AAAAAAAAADJTdXNwZW5kIG9yIGFjdGl2YXRlIGEgcGFydGljaXBhbnQgKHJlZ3VsYXRvciBvbmx5KQAAAAAAFnNldF9wYXJ0aWNpcGFudF9hY3RpdmUAAAAAAAMAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAALcGFydGljaXBhbnQAAAAAEwAAAAAAAAAGYWN0aXZlAAAAAAABAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAEhBcHByb3ZlIGFuIGFjdGl2ZSByZWd1bGF0b3IgcmVtb3ZhbCBwcm9wb3NhbCAobXVsdGktc2lnbmF0dXJlIHRocmVzaG9sZCkAAAAYYXBwcm92ZV9yZW1vdmVfcmVndWxhdG9yAAAAAgAAAAAAAAAGY2FsbGVyAAAAAAATAAAAAAAAAAtwcm9wb3NhbF9pZAAAAAAGAAAAAQAAA+kAAAPtAAAAAAAAAAM=",
        "AAAAAAAAAD9Qcm9wb3NlIHJlbW92YWwgb2YgYSByZWd1bGF0b3IgdW5kZXIgbXVsdGktc2lnbmF0dXJlIGdvdmVybmFuY2UAAAAAGHByb3Bvc2VfcmVtb3ZlX3JlZ3VsYXRvcgAAAAIAAAAAAAAABmNhbGxlcgAAAAAAEwAAAAAAAAATcmVndWxhdG9yX3RvX3JlbW92ZQAAAAATAAAAAQAAA+kAAAAGAAAAAw==",
        "AAAAAQAAACpCYXRjaCBkZXRhaWxzIHN0b3JlZCBpbiBwZXJzaXN0ZW50IHN0b3JhZ2UAAAAAAAAAAAAFQmF0Y2gAAAAAAAAMAAAAAAAAAAhiYXRjaF9pZAAAA+4AAAAgAAAAAAAAAApjcmVhdGVkX2F0AAAAAAAGAAAAAAAAAA9jdXJyZW50X2N1c3RvZHkAAAAAEwAAAAAAAAAPZGlzcGVuc2VkX3BhY2tzAAAAAAQAAAAAAAAAEGV4cGlyeV90aW1lc3RhbXAAAAAGAAAAAAAAAAxtYW51ZmFjdHVyZXIAAAATAAAAAAAAAAttZXJrbGVfcm9vdAAAAAPuAAAAIAAAAAAAAAANbWV0YWRhdGFfaGFzaAAAAAAAA+4AAAAgAAAAAAAAAA5wYWNrYWdpbmdfc3BlYwAAAAAH0AAAAA1QYWNrYWdpbmdTcGVjAAAAAAAAAAAAAA1yZWNhbGxfcmVhc29uAAAAAAAD6AAAABEAAAAAAAAABnN0YXR1cwAAAAAH0AAAAAtCYXRjaFN0YXR1cwAAAAAAAAAADnRvdGFsX3F1YW50aXR5AAAAAAAE",
        "AAAABAAAABpDb250cmFjdCBlcnJvciBkZWZpbml0aW9ucwAAAAAAAAAAAAVFcnJvcgAAAAAAABoAAAAAAAAAEkFscmVhZHlJbml0aWFsaXplZAAAAAAAAQAAAAAAAAAOTm90SW5pdGlhbGl6ZWQAAAAAAAIAAAAAAAAADFVuYXV0aG9yaXplZAAAAAMAAAAAAAAADkNvbnRyYWN0UGF1c2VkAAAAAAAEAAAAAAAAABNQYXJ0aWNpcGFudE5vdEZvdW5kAAAAAAUAAAAAAAAAE1BhcnRpY2lwYW50SW5hY3RpdmUAAAAABgAAAAAAAAAYUGFydGljaXBhbnRBbHJlYWR5RXhpc3RzAAAABwAAAAAAAAALSW52YWxpZFJvbGUAAAAACAAAAAAAAAASQmF0Y2hBbHJlYWR5RXhpc3RzAAAAAAAJAAAAAAAAAA1CYXRjaE5vdEZvdW5kAAAAAAAACgAAAAAAAAAMQmF0Y2hFeHBpcmVkAAAACwAAAAAAAAANQmF0Y2hSZWNhbGxlZAAAAAAAAAwAAAAAAAAAEkludmFsaWRNZXJrbGVQcm9vZgAAAAAADQAAAAAAAAASUHJvb2ZEZXB0aEV4Y2VlZGVkAAAAAAAOAAAAAAAAABZTZXJpYWxBbHJlYWR5RGlzcGVuc2VkAAAAAAAPAAAAAAAAABFJbnZhbGlkU3RyaXBJbmRleAAAAAAAABAAAAAAAAAAFVN0cmlwQWxyZWFkeURpc3BlbnNlZAAAAAAAABEAAAAAAAAAFUJhdGNoUXVhbnRpdHlFeGNlZWRlZAAAAAAAABIAAAAAAAAAE05vdEN1cnJlbnRDdXN0b2RpYW4AAAAAEwAAAAAAAAAWUmVndWxhdG9yQWxyZWFkeUV4aXN0cwAAAAAAFAAAAAAAAAARUmVndWxhdG9yTm90Rm91bmQAAAAAAAAVAAAAAAAAABBQcm9wb3NhbE5vdEZvdW5kAAAAFgAAAAAAAAAMQWxyZWFkeVZvdGVkAAAAFwAAAAAAAAAXUHJvcG9zYWxBbHJlYWR5RXhlY3V0ZWQAAAAAGAAAAAAAAAAZQ2Fubm90UmVtb3ZlTGFzdFJlZ3VsYXRvcgAAAAAAABkAAAAAAAAAEUludmFsaWRQYXJhbWV0ZXJzAAAAAAAAGg==",
        "AAAAAgAAABVDb250cmFjdCBzdG9yYWdlIGtleXMAAAAAAAAAAAAAB0RhdGFLZXkAAAAADAAAAAAAAAAAAAAABUFkbWluAAAAAAAAAAAAAAAAAAAISXNQYXVzZWQAAAABAAAAAAAAAAlSZWd1bGF0b3IAAAAAAAABAAAAEwAAAAAAAAAAAAAADlJlZ3VsYXRvckNvdW50AAAAAAAAAAAAAAAAAA9Qcm9wb3NhbENvdW50ZXIAAAAAAQAAAAAAAAARUmVndWxhdG9yUHJvcG9zYWwAAAAAAAABAAAABgAAAAEAAAAAAAAADVByb3Bvc2FsVm90ZWQAAAAAAAACAAAABgAAABMAAAABAAAAAAAAAAtQYXJ0aWNpcGFudAAAAAABAAAAEwAAAAEAAAAAAAAABUJhdGNoAAAAAAAAAQAAA+4AAAAgAAAAAQAAAAAAAAAPU2VyaWFsRGlzcGVuc2VkAAAAAAIAAAPuAAAAIAAAA+4AAAAgAAAAAQAAAAAAAAAPRGlzcGVuc2VkU3RyaXBzAAAAAAIAAAPuAAAAIAAAA+4AAAAgAAAAAQAAAAAAAAASU3VzcGljaW91c1JlcG9ydGVkAAAAAAACAAAD7gAAACAAAAPuAAAAIA==",
        "AAAAAwAAAClMaWZlLWN5Y2xlIHN0YXR1cyBvZiBhIG1hbnVmYWN0dXJlZCBiYXRjaAAAAAAAAAAAAAALQmF0Y2hTdGF0dXMAAAAAAgAAAAAAAAAGQWN0aXZlAAAAAAABAAAAAAAAAAhSZWNhbGxlZAAAAAI=",
        "AAAAAQAAAChQYXJ0aWNpcGFudCByZWdpc3RyYXRpb24gcmVjb3JkIG9uLWNoYWluAAAAAAAAAAtQYXJ0aWNpcGFudAAAAAAFAAAAAAAAAAZhY3RpdmUAAAAAAAEAAAAAAAAAB2FkZHJlc3MAAAAAEwAAAAAAAAANbWV0YWRhdGFfaGFzaAAAAAAAA+4AAAAgAAAAAAAAAA1yZWdpc3RlcmVkX2F0AAAAAAAABgAAAAAAAAAEcm9sZQAAB9AAAAAPUGFydGljaXBhbnRSb2xlAA==",
        "AAAAAQAAADtQYWNrYWdpbmcgc3BlY2lmaWNhdGlvbnMgZm9yIGJsaXN0ZXIgcGFja3Mgb3IgYnVsayBwYWNrYWdlcwAAAAAAAAAADVBhY2thZ2luZ1NwZWMAAAAAAAACAAAAOFRvdGFsIGJsaXN0ZXIgc3RyaXBzIHBlciBwYWNrIChkZWZhdWx0IDEgZm9yIGJvdHRsZS9ib3gpAAAADHRvdGFsX3N0cmlwcwAAAAQAAAArTnVtYmVyIG9mIGluZGl2aWR1YWwgZG9zYWdlIHVuaXRzIHBlciBzdHJpcAAAAAAPdW5pdHNfcGVyX3N0cmlwAAAAAAQ=",
        "AAAAAwAAADVSb2xlIGFzc2lnbmVkIHRvIGF1dGhvcml6ZWQgc3VwcGx5IGNoYWluIHBhcnRpY2lwYW50cwAAAAAAAAAAAAAPUGFydGljaXBhbnRSb2xlAAAAAAQAAAAAAAAADE1hbnVmYWN0dXJlcgAAAAEAAAAAAAAAC0Rpc3RyaWJ1dG9yAAAAAAIAAAAAAAAACFBoYXJtYWN5AAAAAwAAAAAAAAAJUmVndWxhdG9yAAAAAAAABA==",
        "AAAAAQAAADhNdWx0aS1zaWduYXR1cmUgcHJvcG9zYWwgcmVjb3JkIGZvciByZW1vdmluZyBhIHJlZ3VsYXRvcgAAAAAAAAARUmVndWxhdG9yUHJvcG9zYWwAAAAAAAAGAAAAAAAAAA9hcHByb3ZhbHNfY291bnQAAAAABAAAAAAAAAAKY3JlYXRlZF9hdAAAAAAABgAAAAAAAAAIZXhlY3V0ZWQAAAABAAAAAAAAAAtwcm9wb3NhbF9pZAAAAAAGAAAAAAAAAAhwcm9wb3NlcgAAABMAAAAAAAAAEHRhcmdldF9yZWd1bGF0b3IAAAAT",
        "AAAAAwAAACVWZXJpZmljYXRpb24gdmVyZGljdCBmb3IgYSBwYWNrIHF1ZXJ5AAAAAAAAAAAAABJWZXJpZmljYXRpb25SZXN1bHQAAAAAAAUAAAAAAAAACUF1dGhlbnRpYwAAAAAAAAEAAAAAAAAAB0V4cGlyZWQAAAAAAgAAAAAAAAAIUmVjYWxsZWQAAAADAAAAAAAAAApTdXNwaWNpb3VzAAAAAAAEAAAAAAAAAAdJbnZhbGlkAAAAAAU=" ]),
      options
    )
  }
  public readonly fromJSON = {
    pause: this.txFromJSON<Result<void>>,
        unpause: this.txFromJSON<Result<void>>,
        get_admin: this.txFromJSON<Option<string>>,
        get_batch: this.txFromJSON<Option<Batch>>,
        is_paused: this.txFromJSON<boolean>,
        initialize: this.txFromJSON<Result<void>>,
        get_custody: this.txFromJSON<Option<string>>,
        verify_pack: this.txFromJSON<VerificationResult>,
        get_proposal: this.txFromJSON<Option<RegulatorProposal>>,
        is_regulator: this.txFromJSON<boolean>,
        recall_batch: this.txFromJSON<Result<void>>,
        add_regulator: this.txFromJSON<Result<void>>,
        dispense_pack: this.txFromJSON<Result<void>>,
        register_batch: this.txFromJSON<Result<void>>,
        get_participant: this.txFromJSON<Option<Participant>>,
        transfer_custody: this.txFromJSON<Result<void>>,
        report_suspicious: this.txFromJSON<Result<void>>,
        get_regulator_count: this.txFromJSON<u32>,
        is_serial_dispensed: this.txFromJSON<boolean>,
        get_dispensed_strips: this.txFromJSON<u32>,
        register_participant: this.txFromJSON<Result<void>>,
        is_suspicious_reported: this.txFromJSON<boolean>,
        set_participant_active: this.txFromJSON<Result<void>>,
        approve_remove_regulator: this.txFromJSON<Result<void>>,
        propose_remove_regulator: this.txFromJSON<Result<u64>>
  }
}