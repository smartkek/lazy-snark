//CONNECT FLUENCE
var contractInstance;

$(document).ready(function() {

    // CONNECT WEB3
    if (typeof web3 !== 'undefined') {
        // Use Mist/MetaMask's provider
        let web3js = new Web3(web3.currentProvider);
    } else {
        console.log('No web3? You should consider trying MetaMask!');
        // fallback - use your fallback strategy (local node / hosted node + in-dapp id mgmt / fail)
        let web3js = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));
    }

    //console.log(web3js);

    var controllerAddress = '0x64748b0c0b28079cf88345c1dd992d543d9f028a';
    let ControllerAbi = '[\n' +
        '\t{\n' +
        '\t\t"constant": false,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "id",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "finzalize",\n' +
        '\t\t"outputs": [],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "nonpayable",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": true,\n' +
        '\t\t"inputs": [],\n' +
        '\t\t"name": "last5Timestamps",\n' +
        '\t\t"outputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "result",\n' +
        '\t\t\t\t"type": "uint256[5]"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "view",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": true,\n' +
        '\t\t"inputs": [],\n' +
        '\t\t"name": "verifier",\n' +
        '\t\t"outputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "",\n' +
        '\t\t\t\t"type": "address"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "view",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": true,\n' +
        '\t\t"inputs": [],\n' +
        '\t\t"name": "stake",\n' +
        '\t\t"outputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "view",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": false,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"components": [\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "input",\n' +
        '\t\t\t\t\t\t"type": "uint256[5]"\n' +
        '\t\t\t\t\t}\n' +
        '\t\t\t\t],\n' +
        '\t\t\t\t"name": "data",\n' +
        '\t\t\t\t"type": "tuple"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"components": [\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "a",\n' +
        '\t\t\t\t\t\t"type": "uint256[2]"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "b",\n' +
        '\t\t\t\t\t\t"type": "uint256[2][2]"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "c",\n' +
        '\t\t\t\t\t\t"type": "uint256[2]"\n' +
        '\t\t\t\t\t}\n' +
        '\t\t\t\t],\n' +
        '\t\t\t\t"name": "proof",\n' +
        '\t\t\t\t"type": "tuple"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "submit",\n' +
        '\t\t"outputs": [],\n' +
        '\t\t"payable": true,\n' +
        '\t\t"stateMutability": "payable",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": true,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "tasks",\n' +
        '\t\t"outputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"components": [\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "input",\n' +
        '\t\t\t\t\t\t"type": "uint256[5]"\n' +
        '\t\t\t\t\t}\n' +
        '\t\t\t\t],\n' +
        '\t\t\t\t"name": "data",\n' +
        '\t\t\t\t"type": "tuple"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"components": [\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "a",\n' +
        '\t\t\t\t\t\t"type": "uint256[2]"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "b",\n' +
        '\t\t\t\t\t\t"type": "uint256[2][2]"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "c",\n' +
        '\t\t\t\t\t\t"type": "uint256[2]"\n' +
        '\t\t\t\t\t}\n' +
        '\t\t\t\t],\n' +
        '\t\t\t\t"name": "proof",\n' +
        '\t\t\t\t"type": "tuple"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "submitter",\n' +
        '\t\t\t\t"type": "address"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "timestamp",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "status",\n' +
        '\t\t\t\t"type": "uint8"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "view",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": true,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "id",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "taskDataById",\n' +
        '\t\t"outputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "data",\n' +
        '\t\t\t\t"type": "uint256[13]"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "view",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": false,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "id",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "challenge",\n' +
        '\t\t"outputs": [],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "nonpayable",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"constant": true,\n' +
        '\t\t"inputs": [],\n' +
        '\t\t"name": "tasksNum",\n' +
        '\t\t"outputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"name": "",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"payable": false,\n' +
        '\t\t"stateMutability": "view",\n' +
        '\t\t"type": "function"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"anonymous": false,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"indexed": true,\n' +
        '\t\t\t\t"name": "sender",\n' +
        '\t\t\t\t"type": "address"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"indexed": false,\n' +
        '\t\t\t\t"name": "index",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"components": [\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"components": [\n' +
        '\t\t\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t\t\t"name": "input",\n' +
        '\t\t\t\t\t\t\t\t"type": "uint256[5]"\n' +
        '\t\t\t\t\t\t\t}\n' +
        '\t\t\t\t\t\t],\n' +
        '\t\t\t\t\t\t"name": "data",\n' +
        '\t\t\t\t\t\t"type": "tuple"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"components": [\n' +
        '\t\t\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t\t\t"name": "a",\n' +
        '\t\t\t\t\t\t\t\t"type": "uint256[2]"\n' +
        '\t\t\t\t\t\t\t},\n' +
        '\t\t\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t\t\t"name": "b",\n' +
        '\t\t\t\t\t\t\t\t"type": "uint256[2][2]"\n' +
        '\t\t\t\t\t\t\t},\n' +
        '\t\t\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t\t\t"name": "c",\n' +
        '\t\t\t\t\t\t\t\t"type": "uint256[2]"\n' +
        '\t\t\t\t\t\t\t}\n' +
        '\t\t\t\t\t\t],\n' +
        '\t\t\t\t\t\t"name": "proof",\n' +
        '\t\t\t\t\t\t"type": "tuple"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "submitter",\n' +
        '\t\t\t\t\t\t"type": "address"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "timestamp",\n' +
        '\t\t\t\t\t\t"type": "uint256"\n' +
        '\t\t\t\t\t},\n' +
        '\t\t\t\t\t{\n' +
        '\t\t\t\t\t\t"name": "status",\n' +
        '\t\t\t\t\t\t"type": "uint8"\n' +
        '\t\t\t\t\t}\n' +
        '\t\t\t\t],\n' +
        '\t\t\t\t"indexed": false,\n' +
        '\t\t\t\t"name": "task",\n' +
        '\t\t\t\t"type": "tuple"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "Submitted",\n' +
        '\t\t"type": "event"\n' +
        '\t},\n' +
        '\t{\n' +
        '\t\t"anonymous": false,\n' +
        '\t\t"inputs": [\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"indexed": true,\n' +
        '\t\t\t\t"name": "challenger",\n' +
        '\t\t\t\t"type": "address"\n' +
        '\t\t\t},\n' +
        '\t\t\t{\n' +
        '\t\t\t\t"indexed": false,\n' +
        '\t\t\t\t"name": "index",\n' +
        '\t\t\t\t"type": "uint256"\n' +
        '\t\t\t}\n' +
        '\t\t],\n' +
        '\t\t"name": "Challenged",\n' +
        '\t\t"type": "event"\n' +
        '\t}\n' +
        ']';
    ControllerAbi = JSON.parse(ControllerAbi);
    //console.log(ControllerAbi);
    let ControllerContract = web3.eth.contract(ControllerAbi);
    //console.log(ControllerContract);
    contractInstance = ControllerContract.at(controllerAddress);
    //console.log(contractInstance);
/*
    // save fluence to global variable, so it can be accessed from Developer Console
    window.fluence = fluence;

    // convert result to a string
    window.getResultAsString = function (result) {
        return result.result().then((r) => r.asString())
    };

    window.logResultAsString = function (result) {
        return getResultAsString(result).then((r) => console.log(r))
    };
*/


    contractInstance.tasksNum(function (err, result) {
        console.log(result);
        let maxLen = Math.min(result, 5);
        for (let i = 0; i < maxLen; i++) {
            let data = result - 1 - i;
            let fluenceResponse = session.request(`{"action": "Check", "proof_id": ${data}}`);

            $('#state-id-' + i).text(result - 1 - i);
            if (fluenceResponse.hasOwnProperty('verified')) {
                if (fluenceResponse.verified) {
                    // все хорошо - мы проверили в флюенсе
                    $('#challenge-' + i).prop('disabled', true);
                } else {
                    // мы проверили, пруф неправильный
                    $('#challenge-' + i).text('Challenge on Ethereum!')
                }
            }
        }
    });



    /*window.onload = function () {

        // address to Fluence contract in Ethereum blockchain. Interaction with blockchain created by MetaMask or with local Ethereum node
        let contractAddress = "0xeFF91455de6D4CF57C141bD8bF819E5f873c1A01";

        // set ethUrl to `undefined` to use MetaMask instead of Ethereum node
        let ethUrl = "http://geth.fluence.one:8545/";

        // application to interact with that stored in Fluence contract
        let appId = "218";

        // create a session between client and backend application, and then join the game
        fluence.connect(contractAddress, appId, ethUrl).then((s) => {
            console.log("Session created");
            window.session = s;
        }).then(() => join());


        // send request to list last txses

        for (let i = 0; i < 5; i++) {
            let id = document.getElementById('state-id-' + i).textContent;
            contractInstance.giveDataFromId(id, function (err, result) {

                let fluenceResponse = session.request(`{ "player_id": ${id}, "action": "GetBalance"}`);
                $('#state-status-fluence-' + i).text(fluenceResponse);
                if (fluenceResponse == true) {
                    $('challenge-' + i).prop('disabled', true);
                } else {
                    $('challenge-' + i).text('Challenge on Ethereum!')
                }
            });
        }

    };*/

});

$('button').click(function () {
    //console.log($(this)[0].id.slice(-1));
    if ($(this)[0].textContent === 'Challenge on Fluence!') {
        let id = $(this)[0].id.slice(-1);
        let data = $('#state-id-' + id)[0].textContent;
        console.log(data);
        //console.log(contractInstance);
        contractInstance.taskDataById(data, function (err, result) {
            //console.log(err);
            console.log(result);
            let public_par = result.slice(0, 5);
            let proof = result.slice(5, 13);
            let fluenceResponse = session.request(`{"action": "Verify", "proof_id": ${data}, "public_par": ${public_par}, "proof": ${proof}}`);
            console.log(fluenceResponse);
            $('#state-status-fluence-' + i).text(result);
            if (fluenceResponse.result) {
                // все хорошо - мы проверили в флюенсе
                $('#challenge-' + i).prop('disabled', true);
            } else {
                // мы проверили, пруф неправильный
                $('#challenge-' + i).text('Challenge on Ethereum!')
            }
        });
    } else {
        challengeEthereum(data);
    }
});

function challengeEthereum(jobId) {
    contractInstance.challenge.sendTransaction(jobId, function (err, txHash) {
        $('#challenge-' + jobId).text('See tx on Etherscan!').attr("href", "https://ropsten.etherscan.io/tx/" + txHash);
    });
}


import * as fluence from "fluence";
