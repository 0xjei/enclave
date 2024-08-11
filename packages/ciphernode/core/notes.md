- [ ] Setup mock EventListner system
- [ ] EnclaveEvm dispatches CommitteeSelected
- [ ] both Ciphernode and Aggregator listens for the event and runs keyshare handler
- [ ] Aggregator listens for all other keyshares and aggregates them
- [ ] Aggregator calls EnclaveEvm method to report back to the SC

# Questions

- Can we use a unified signature algorhythm to sign all events? ^
  - Should this be ECDSA for ethereum compatability
  - Should this be EDDSA over the BabyJubJub curve to be compatible with Zero Knowledge Systems

# Event dispatch

Every time an event gets dispatched it needs a global incremental id that is the hash of the previous event. If two events have the same id. We can use a hashclock as a VDF or we can publish events to an existing blockchain (eg. gnosis chain) to act as an event ordering point. in this way nodes can publish events first to gnosis chain gaining an event id.

All nodes in the system have a publickey and sign all events. If we use a low gas chain then the signature is embedded within the event dispatch

# Ciphernode registration

- There are two large
- SC register/stake emits CiphernodeRegisteredEvent
- event has: id, address, bjpubkey
  They register their bjj pk with the smart contract as an identity register. This validates the caller and emits an Event with the bjj pk and the address.
- BJJ public key can be derived from ecdsa private key - assuming we have access on the client -- if not we can derive it by deterministically signing of some known data. "**GNOSIS_ENCLAVE**"
- We use BJJ keys to sign data so that we can create signatures that can be verified within a ZKP.

We can manage a huge node list within solidity like so:

- lists are managed in solidity like so:
  - `Node` objects represents the nodes.`{ isRegistered, rank }`.
  - A mapping holds adddresses to Node objects. `mapping(address => Node) nodes`.
  - A list holds nodes within their sortition rank `address[] ranking`.
  - Registering a node involves adding the node to the mapping and the array. Events dispatched: `NodeRegistered(address, publicKey, rank, newRankListLength)`.
  - Deregistering a node involves locating the node according to it's rank within the ranking list locating the last node and moving the last node within the rank to the location of the node adjusting their rank fields accordingly.
  - This will dispatch `NodeDeregistered(removeAddress, removeRank, lastRankAddress, newRankListLength)`

In this way we can manage a large list of registered nodes onchain without huge gas costs.

# Requester requesting computation

When a requester requests a computation they provide the minimal bond to the Enclave smart contract and dipspatch `ComputationRequested(e3_id,computation_type,execution_module_type,ciphernode_group_length,ciphernode_threshold,input_deadline,availability_duration,sortition_seed)`.

# Sortition

A sorter is aware of registered nodes and can maintain a list of filters

Each node can calculate it's rank deterministically

Use the fisher-yates shuffle.

We need a filtered list maintained by the Registry and for the node to know:

- It's rank in the list.
- The length of the list.

Registry

- Maintains list of registered ciphernodes from dispatched evm events
- sc event: "ComputationRequested" includes a random seed used to determine the nodes in the comittee
- uses random seed to create a random list of ciphernodes
- deterministic process can be run on all nodes.

By only sending the rank list length and random seed to the node we save on traffic and need not publish the entire list.

# Aggregation

When an aggregation node receives a `ComputationRequested` event they take  
Nodes compete to aggregate the public key for the committee. Nodes sign their share as they publish it.^ This dispatches a `KeyshareCreated(PublicKeyShare,Signature)` event. The `PublicKeyShare` can be validated by ensuring the Signature matches the given publickey. The keyshare can then be aggregated
