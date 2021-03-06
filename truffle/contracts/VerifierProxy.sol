pragma solidity 0.5.10;
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
        bytes memory payload = abi.encodeWithSelector(verifier.verifyTx.selector, proof.a, proof.b, proof.c, data.input);
        (bool success, bytes memory r) = address(verifier).call(payload);
        return success && abi.decode(r, (bool));
        // return verifier.verifyTx(proof.a, proof.b, proof.c, data.input);
    }
}