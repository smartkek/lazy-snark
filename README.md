# LAZY SNARK
### trustless off-chain zk-proof verification
##Abstract
In Ethereum, it is expensive to check zk-proofs on-chain, so we propose to use Fluence to do heavy-lifting off-chain and only go on-chain to challenge incorrect data/proof combination.
##Why
Let us say, we need to verify zk-proofs in Ethereum smart contract. The problem is that zk-proof verification is a heavy computational task and thus costs a lot of gas. As a result, checking proofs on-chain is not only expensive, but also takes a lot of space in the block. This may lead to serious problems, e.g. in case of mass exit.
##What
We suggest checking proofs on Fluence instead. This option does not has gas problem, is much cheaper, but is also trustless. 
......................................................
##How it works
The process includes the following entities:
-Ethereum smart contract that stores (data, proof) pairs and implements on-chain proof verification. In case the proof is not correct, the smart contract rewards the user who checked this proof with ether.
-Operator who uploads (data, proof) pairs to the smart contract.
-Fluence instance that also implements proof verification, but off-chain. It also stores all the check results.
-Fisherman aka the user of our system. The fisherman wants to find false proofs to check them in a smart contract and get a reward.

Here is the workflow:
1. The operator uploads (data, proof) to the smart contract.
2. The fisherman takes (data, proof) from the smart contract and uploads it to the Fluence instance.
3. The Fluence instance checkes the proof.
4. a) If the proof is correct, it is stored by the Fluence instance with TRUE flag. Other fishermen can see it an will not check this proof again.
   b) If the proof is false, the fisherman checks the same proof in the smart contract. In that case the fisherman is sure that the proof is FALSE and thus the fisherman will get the reward.

To better understand the workflow, please review the scheme.
