pragma solidity ^0.5.4;
pragma experimental ABIEncoderV2;

import "./IVerifier.sol";
import "./Verifier.sol";


contract VerifierProxy is IVerifier {
    Verifier internal verifier;

    constructor(Verifier ver) public {
        verifier = ver;
    }

    // Truffle gives `UnimplementedFeatureError: Encoding struct from calldata is not yet supported.`
    // that's why function is public and uses memory location modifier 
    function isValid(Data memory data, Proof memory proof) public returns (bool) {
        bytes memory payload = abi.encodeWithSelector(verifier.verifyTx.selector, proof, data);
        (bool success, bytes memory r) = address(verifier).call(payload);
        require(success);
        return abi.decode(r, (bool));
    }
}