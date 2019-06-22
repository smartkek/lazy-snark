pragma solidity ^0.5.4;
pragma experimental ABIEncoderV2;

import "./Structs.sol";

contract IVerifier is Structs {
    function isValid(Data calldata data, Proof calldata proof) external returns (bool);
}
