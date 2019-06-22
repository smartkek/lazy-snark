//CONNECT FLUENCE
import * as fluence from "fluence";

let session;
window.onload = function () {
    let contractAddress = "0xeFF91455de6D4CF57C141bD8bF819E5f873c1A01";

    // set ethUrl to `undefined` to use MetaMask instead of Ethereum node
    let ethUrl = "http://rinkeby.fluence.one:8545/";

    // application to interact with that stored in Fluence contract
    let appId = "264";

    // create a session between client and backend application, and then join the game
    fluence.connect(contractAddress, appId, ethUrl).then((s) => {
        console.log("Session created");
        session = s;
    });
};


var contractInstance;

$(document).ready(function() {
    // window.addEventListener('load', () => {
    //     if (typeof Web3 !== 'undefined') {
    //         let web3js = new Web3(web3.currentProvider);
    //     } else {
    //         console.log('No web3? You should consider trying MetaMask!');
    //         let web3js = new Web3(new Web3.providers.HttpProvider('http://localhost:8080'));
    //     }
    // });

    // console.log('provider');
    // console.log(web3js);

    var controllerAddress = '0x1cca1f0be338c747b11a16aba8d0905251628bdf';
    let ControllerAbi = [
        {
            "constant": true,
            "inputs": [],
            "name": "verifier",
            "outputs": [
                {
                    "name": "",
                    "type": "address"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [],
            "name": "stake",
            "outputs": [
                {
                    "name": "",
                    "type": "uint256"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [
                {
                    "name": "",
                    "type": "uint256"
                }
            ],
            "name": "tasks",
            "outputs": [
                {
                    "components": [
                        {
                            "name": "input",
                            "type": "uint256[5]"
                        }
                    ],
                    "name": "data",
                    "type": "tuple"
                },
                {
                    "components": [
                        {
                            "name": "a",
                            "type": "uint256[2]"
                        },
                        {
                            "name": "b",
                            "type": "uint256[2][2]"
                        },
                        {
                            "name": "c",
                            "type": "uint256[2]"
                        }
                    ],
                    "name": "proof",
                    "type": "tuple"
                },
                {
                    "name": "submitter",
                    "type": "address"
                },
                {
                    "name": "timestamp",
                    "type": "uint256"
                },
                {
                    "name": "status",
                    "type": "uint8"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        },
        {
            "anonymous": false,
            "inputs": [
                {
                    "indexed": true,
                    "name": "sender",
                    "type": "address"
                },
                {
                    "indexed": false,
                    "name": "index",
                    "type": "uint256"
                },
                {
                    "components": [
                        {
                            "components": [
                                {
                                    "name": "input",
                                    "type": "uint256[5]"
                                }
                            ],
                            "name": "data",
                            "type": "tuple"
                        },
                        {
                            "components": [
                                {
                                    "name": "a",
                                    "type": "uint256[2]"
                                },
                                {
                                    "name": "b",
                                    "type": "uint256[2][2]"
                                },
                                {
                                    "name": "c",
                                    "type": "uint256[2]"
                                }
                            ],
                            "name": "proof",
                            "type": "tuple"
                        },
                        {
                            "name": "submitter",
                            "type": "address"
                        },
                        {
                            "name": "timestamp",
                            "type": "uint256"
                        },
                        {
                            "name": "status",
                            "type": "uint8"
                        }
                    ],
                    "indexed": false,
                    "name": "task",
                    "type": "tuple"
                }
            ],
            "name": "Submitted",
            "type": "event"
        },
        {
            "anonymous": false,
            "inputs": [
                {
                    "indexed": true,
                    "name": "challenger",
                    "type": "address"
                },
                {
                    "indexed": false,
                    "name": "index",
                    "type": "uint256"
                }
            ],
            "name": "Challenged",
            "type": "event"
        },
        {
            "constant": true,
            "inputs": [],
            "name": "tasksNum",
            "outputs": [
                {
                    "name": "",
                    "type": "uint256"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        },
        {
            "constant": false,
            "inputs": [
                {
                    "components": [
                        {
                            "name": "input",
                            "type": "uint256[5]"
                        }
                    ],
                    "name": "data",
                    "type": "tuple"
                },
                {
                    "components": [
                        {
                            "name": "a",
                            "type": "uint256[2]"
                        },
                        {
                            "name": "b",
                            "type": "uint256[2][2]"
                        },
                        {
                            "name": "c",
                            "type": "uint256[2]"
                        }
                    ],
                    "name": "proof",
                    "type": "tuple"
                }
            ],
            "name": "submit",
            "outputs": [],
            "payable": true,
            "stateMutability": "payable",
            "type": "function"
        },
        {
            "constant": false,
            "inputs": [
                {
                    "name": "id",
                    "type": "uint256"
                }
            ],
            "name": "challenge",
            "outputs": [],
            "payable": false,
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "constant": false,
            "inputs": [
                {
                    "name": "id",
                    "type": "uint256"
                }
            ],
            "name": "finzalize",
            "outputs": [],
            "payable": false,
            "stateMutability": "nonpayable",
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [
                {
                    "name": "id",
                    "type": "uint256"
                }
            ],
            "name": "taskDataById",
            "outputs": [
                {
                    "name": "data",
                    "type": "uint256[13]"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [],
            "name": "last5Timestamps",
            "outputs": [
                {
                    "name": "result",
                    "type": "uint256[5]"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        },
        {
            "constant": true,
            "inputs": [
                {
                    "name": "id",
                    "type": "uint256"
                }
            ],
            "name": "getDataById",
            "outputs": [
                {
                    "components": [
                        {
                            "components": [
                                {
                                    "name": "input",
                                    "type": "uint256[5]"
                                }
                            ],
                            "name": "data",
                            "type": "tuple"
                        },
                        {
                            "components": [
                                {
                                    "name": "a",
                                    "type": "uint256[2]"
                                },
                                {
                                    "name": "b",
                                    "type": "uint256[2][2]"
                                },
                                {
                                    "name": "c",
                                    "type": "uint256[2]"
                                }
                            ],
                            "name": "proof",
                            "type": "tuple"
                        },
                        {
                            "name": "submitter",
                            "type": "address"
                        },
                        {
                            "name": "timestamp",
                            "type": "uint256"
                        },
                        {
                            "name": "status",
                            "type": "uint8"
                        }
                    ],
                    "name": "task",
                    "type": "tuple"
                }
            ],
            "payable": false,
            "stateMutability": "view",
            "type": "function"
        }
    ];
    let ControllerContract = web3.eth.contract(ControllerAbi);
    contractInstance = ControllerContract.at(controllerAddress);

    contractInstance.tasksNum(function (err, result) {
        console.log(result);
        let maxLen = Math.min(result, 5);
        for (let i = 0; i < maxLen; i++) {
            let data = result - 1 - i;
            // let fluenceResponse = session.request(`{"action": "Check", "proof_id": ${data}}`);
            // console.log(fluenceResponse);
            $('#state-id-' + i).text(result - 1 - i);
            if (fluenceResponse.hasOwnProperty('verified')) {
                if (fluenceResponse.verified) {
                    // все хорошо - мы проверили в флюенсе
                    $('#state-status-fluence-' + i).text('TRUE by Fluence.');
                    $('#challenge-' + i).prop('disabled', true);
                } else {
                    // мы проверили, пруф неправильный
                    $('#state-status-fluence-' + i).text('FALSE by Fluence.');
                    $('#challenge-' + i).text('Challenge on Ethereum!')
                }
            }
        }
    });
});

$('button').click(function () {
    let id = $(this)[0].id.slice(-1);
    let data = $('#state-id-' + id)[0].textContent;

    if ($(this)[0].textContent === 'Challenge on Fluence!') {
        contractInstance.taskDataById(data, function (err, result) {
            let public_par = result.slice(0, 5);
            let proof = result.slice(5, 13);

            let fluenceResponse = session.request(`{"action": "Verify", "proof_id": ${data}, "public_par": ${public_par}, "proof": ${proof}}`);
            console.log(fluenceResponse);
            // let success = fluenceResponse.result === 1;
            let success = false;
            if (success) {
                // все хорошо - мы проверили в флюенсе
                $('#state-status-fluence-' + id).text('TRUE by Fluence.');
                $('#challenge-' + id).prop('disabled', true);
            } else {
                // мы проверили, пруф неправильный
                $('#state-status-fluence-' + id).text('FALSE by Fluence.');
                $('#challenge-' + id).text('Challenge on Ethereum!')
            }
        });
    } else {
        challengeEthereum(data);
    }
});

function challengeEthereum(jobId) {
    console.log(jobId);
    contractInstance.challenge.sendTransaction(jobId, function (err, txHash) {
        $('#challenge-' + jobId).text('See tx on Etherscan!').attr("href", "https://ropsten.etherscan.io/tx/" + txHash);
    });
}
