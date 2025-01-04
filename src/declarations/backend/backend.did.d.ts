import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type BitcoinNetwork = { 'mainnet' : null } |
  { 'regtest' : null } |
  { 'testnet' : null };
export interface InitArgs {
  'commission_receiver' : [] | [Principal],
  'auth' : [] | [Principal],
  'bitcoin_network' : BitcoinNetwork,
}
export interface StartLaunchArgs {
  'x' : [] | [string],
  'fee_per_vbytes' : [] | [bigint],
  'duration' : number,
  'turbo' : boolean,
  'starts_in' : number,
  'logo' : [] | [Uint8Array | number[]],
  'content_type' : [] | [Uint8Array | number[]],
  'divisibility' : number,
  'hard_cap' : bigint,
  'website' : [] | [string],
  'price_per_token' : bigint,
  'soft_cap' : bigint,
  'raise_in' : TokenType,
  'runename' : string,
  'telegram' : [] | [string],
  'total_supply' : bigint,
  'symbol' : [] | [number],
  'openchat' : [] | [string],
}
export type TokenType = { 'Bitcoin' : null };
export interface _SERVICE {
  'start_launch' : ActorMethod<[StartLaunchArgs], undefined>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
