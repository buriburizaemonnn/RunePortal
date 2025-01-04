export const idlFactory = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({
    'commission_receiver' : IDL.Opt(IDL.Principal),
    'auth' : IDL.Opt(IDL.Principal),
    'bitcoin_network' : BitcoinNetwork,
  });
  const TokenType = IDL.Variant({ 'Bitcoin' : IDL.Null });
  const StartLaunchArgs = IDL.Record({
    'x' : IDL.Opt(IDL.Text),
    'fee_per_vbytes' : IDL.Opt(IDL.Nat64),
    'duration' : IDL.Nat8,
    'turbo' : IDL.Bool,
    'starts_in' : IDL.Nat8,
    'logo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'content_type' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'divisibility' : IDL.Nat8,
    'hard_cap' : IDL.Nat64,
    'website' : IDL.Opt(IDL.Text),
    'price_per_token' : IDL.Nat64,
    'soft_cap' : IDL.Nat64,
    'raise_in' : TokenType,
    'runename' : IDL.Text,
    'telegram' : IDL.Opt(IDL.Text),
    'total_supply' : IDL.Nat,
    'symbol' : IDL.Opt(IDL.Nat32),
    'openchat' : IDL.Opt(IDL.Text),
  });
  return IDL.Service({ 'start_launch' : IDL.Func([StartLaunchArgs], [], []) });
};
export const init = ({ IDL }) => {
  const BitcoinNetwork = IDL.Variant({
    'mainnet' : IDL.Null,
    'regtest' : IDL.Null,
    'testnet' : IDL.Null,
  });
  const InitArgs = IDL.Record({
    'commission_receiver' : IDL.Opt(IDL.Principal),
    'auth' : IDL.Opt(IDL.Principal),
    'bitcoin_network' : BitcoinNetwork,
  });
  return [InitArgs];
};
