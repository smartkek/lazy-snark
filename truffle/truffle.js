// require('babel-register');
// require('babel-polyfill');

var HDWalletProvider = require("truffle-hdwallet-provider");


var mnemonicRinkeby = "oxygen crunch note tent verify chicken gossip shield essence runway clinic fortune";
var infuraLinkRinkeby = "https://rinkeby.infura.io/v3/f06b7ded27484b2f8590183576eeec95";



module.exports = {
    networks: {
        rinkeby: {
            provider: function() {
                return new HDWalletProvider(mnemonicRinkeby, infuraLinkRinkeby)
            },
            network_id: "4"
        }
    },
    compilers: {
        solc: {
            version: "0.5.4",
            settings: {
                optimizer: {
                    enabled: true,
                    runs: 200
                }
            }
        }
    }
};
