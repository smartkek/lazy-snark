pragma solidity ^0.5.4;

interface Structs {
    struct Data {
        uint[5] input;
    }
    
    struct Proof {
        uint[2]  a;
        uint[2][2]  b;
        uint[2]  c;
    }
}
