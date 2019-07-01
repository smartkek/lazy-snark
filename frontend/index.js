//CONNECT FLUENCE
import * as fluence from "fluence";


window.getResultAsString = function (result) {
    return result.result().then((r) => r.asString())
};

var contractInstance;

$(document).ready(async function() {
    let contractAddress = "0xeFF91455de6D4CF57C141bD8bF819E5f873c1A01";

    // set ethUrl to `undefined` to use MetaMask instead of Ethereum node
    let ethUrl = "http://rinkeby.fluence.one:8545/";

    // application to interact with that stored in Fluence contract
    let appId = "269";

    // create a session between client and backend application, and then join the game
    await fluence.connect(contractAddress, appId, ethUrl).then((s) => {
        console.log("Session created");
        window.session = s;
    });

    var lazyAddress = '0x4812b51D4420Da87C3cD78a77048328cd6914f27';
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
    contractInstance = ControllerContract.at(lazyAddress);
    window.ethereum.enable();

    contractInstance.tasksNum(function (err, result) {
        let maxLen = Math.min(result, 5);
        for (let i = 0; i < maxLen; i++) {
            let data = result - 1 - i;
            $('#state-id-' + i).text(data);
            let fluenceResponse_check = session.request(`{"action": "Check", "proof_id": ${data}}`);
            getResultAsString(fluenceResponse_check).then(function (str) {
                let fluenceResponse = JSON.parse(str);
                console.log(fluenceResponse);
                if (fluenceResponse.hasOwnProperty('verifed')) {
                    if (fluenceResponse.verifed) {
                        // все хорошо - мы проверили в флюенсе
                        $('#state-status-fluence-' + i).text('TRUE by Fluence.');
                        $('#challenge-' + i).prop('disabled', true);
                    } else {
                        // мы проверили, пруф неправильный
                        $('#state-status-fluence-' + i).text('FALSE by Fluence.');
                        $('#challenge-' + i).text('Challenge on Ethereum!')
                    }
                } else {
                    console.log('Task N ' + data + ' is not checked on Fluence!')
                }
            });
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

            let fluenceResponse_ver = session.request(`{"action": "Verify", "proof_id": ${data}, "public_par": [${public_par}], "proof": [${proof}]}`);
            getResultAsString(fluenceResponse_ver).then(function (str) {
                let fluenceResponse = JSON.parse(str);
                let success = fluenceResponse.result === 1;
                console.log(fluenceResponse);
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
        });
    } else if ($(this)[0].textContent === 'Challenge on Ethereum!') {
        challengeEthereum(id, data);
    } else {

    }
});

function challengeEthereum(id, data) {
    console.log('Challenging task N ' + data + ' on Ethereum!');
    contractInstance.challenge.sendTransaction(data, function (err, txHash) {
        $('#challenge-' + id).text('See tx on Etherscan!');
        $('#challenge-' + id).attr("onclick", "window.open('https://rinkeby.etherscan.io/tx/" + txHash + "')");
    });
}
