pragma solidity ^0.5.4;
pragma experimental ABIEncoderV2;

interface Structs {
    struct Data {
        uint x;
    }
    
    struct Proof {
        uint x;
    }

}

contract Verifier is Structs {
    function isValid(Data calldata data, Proof calldata proof) external returns (bool);
}

contract Lazy is Structs {
    event Submitted(address indexed sender, uint256 index, Task task);
    event Challenged(address indexed challenger, uint256 index);


    enum Status {UNCHECKED, VALID, INVALID, FINALIZED} 
    struct Task {
        Data data;
        Proof proof;
        address payable submitter;
        uint96 timestamp;
        Status status;
    }
    
    Task[] public tasks;
    
    uint256 public stake;
    Verifier public verifier;

    /// @dev This function submits data.
    /// @param data - public inptut for zkp
    /// @param proof - proof that verifies input
    function submit(Data calldata data, Proof calldata proof) external payable {
        require(msg.value == stake);

        Task memory task = Task(data, proof, msg.sender, uint96(now), Status.UNCHECKED);
        uint index = tasks.push(task);

        emit Submitted(msg.sender, index, task);
    }

    /// @dev This function challenges a submission by calling the validation function.
    /// @param id The id of the submission to challenge.
    function challenge(uint id) external {
        Task storage task = tasks[id];
        require(now < task.timestamp + 1 weeks);
        require(task.status == Status.UNCHECKED);
        
        if (verifier.isValid(task.data, task.proof)) {
            task.status = Status.VALID;
            task.submitter.transfer(stake);
        } else {
            task.status = Status.INVALID;
            msg.sender.transfer(stake);
        }
        
    
        // пруф не подходит, на это надо реагировать

        emit Challenged(msg.sender, id);
    }
    
    function finzalize(uint id) external {
        Task storage task = tasks[id];
        require(now > task.timestamp + 1 weeks);
        require(task.status == Status.UNCHECKED);
        
        task.status = Status.FINALIZED;
        msg.sender.transfer(stake);
    }
}
    
