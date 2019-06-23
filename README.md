# LAZY SNARK: trustless off-chain zk-proof verification.
## Abstract
In Ethereum, it is expensive to check zero-knowledge proofs on-chain. So, we propose to use Fluence to do heavy-lifting off-chain and only go on-chain to challenge incorrect proofs. Our project should help exisiting Ethereum projects that rely on zk-proofs to achieve privacy, scalability, and trustlessness.
## Why
Let us say, there is a project that needs to verify zk-proofs in Ethereum smart contract. The problem is that zk-proof verification is a heavy computational task and thus costs a lot of gas. As a result, checking proofs on-chain is expensive, and is susceptible to network congestion. 
## What
We suggest checking proofs on Fluence instead. This option does not has gas problem. Thus, it is much cheaper. Also, it won't consume all the gas in the block. Besides, it is trustless and the results of the checks are public.
## How it works
The process includes the following entities:
- Ethereum smart contract that stores (data, proof) pairs and implements on-chain proof verification. In case the proof is not correct, the smart contract rewards the user who challenges the invalid proof with ether.
- Proof supplier who uploads (data, proof) pairs to the smart contract. The proof supplier stakes ether to the smart contract. In case the proof supplier provides an invalid proof, the proof supplier is punished: a part of the stake is given to the one who chellenged the proof as a reward.
- Fluence back end that implements off-chain proof verification. It also stores proof verification results.
- Ethereum project user aka proof consumer. The user checks whether the proof supplier has provided valid proofs and challenges invalid ones using smart contract to get a reward.
- Arweave front end. The user performs all the actions via the front end. Also, proof results from the Fluence back end are displayed in the front end.

Here is the workflow:
1. The proof supplier uploads (data, proof) to the smart contract.
2. The user takes (data, proof) from the smart contract and sends it to the back end.
3. The back end checkes the proof.
4. The following actions depend on the result of the check:
- a. If the proof is valid, it is stored by the back end with TRUE flag. Other users can see it in the front end and will not check this proof again.
- b. If the proof is invalid, it is stored by the back end with FALSE flag. The user challenges this proof in the smart contract. In that case the user is sure that the proof is FALSE and thus the user will get the reward.

To better understand the workflow, please review the scheme.

![Image](Scheme.png "Scheme")

## Benefits
LAZY SNARK (not to be confused with Lady Stark) provides the following benefits compared to checking zk-proofs on-chain:
- It doesn't require much gas. In case of e.g. mass exit the zk-proofs verifications will take all the gas in the block. LAZY SNARK won't.
- It is ~10 times cheaper than verifying zk-proofs in Ethereum smart contract. Checking zk-proof on-chain costs ~$1 (gasprice and ETH price on June 23, 2019). Checking them in Fluence costs ~$0.01. Since we still need to put data and proofs on-chain, the whole system operation will cost 10 times less than checking the proofs on-chain.

LAZY SNARK provides the following benefits compared to checking zk-proofs locally:
- It is trustless unlike checking zk-proofs locally. All the proof check results are available on Fluence via Arweave front end.
- The results are public, so everyone will be able to see which proofs are valid and which proof suppliers are honest.
- The results are public, so the users who seek for invalid proofs won't check the proofs that has already been checked.

## Use cases
LAZY SNARK can be used in the following cases (and not only in these):
- Plasma implementations that require zk-proofs.
- Mixers.
- Private money like ZkDai.
- Games.
- Many other systems that use zk-proofs. For now, they are mostly limited to money-like systems, hence the examples above. However, it is only the matter of imagination what other use cases need zk-proofs, scalability, and trustlessness.

## Build instructions
To be provided :) 