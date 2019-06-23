// require('babel-register');
// require('babel-polyfill');

var HDWalletProvider = require("truffle-hdwallet-provider");

var mnemonicRopsten = "...";
var infuraLinkRopsten = "...";
var deployerAddressRopsten = "...";

var mnemonicKovan = "...";
var infuraLinkKovan = "...";
var deployerAddressKovan = "...";
var addressIndex = 0; // address index in MetaMask


module.exports = {
    networks: {
        development: {
            host: "localhost",
            network_id: "*",
            port: 8545,
        },
        coverage: {
            host: "localhost",
            network_id: "*",
            port: 8555,         // <-- If you change this, also set the port option in .solcover.js.
            gas: 0xfffffffffff, // <-- Use this high gas value
            gasPrice: 0x01      // <-- Use this low gas price
        },
        ropsten: {
            provider: function() {
                return new HDWalletProvider(mnemonicRopsten, infuraLinkRopsten, addressIndex);
            },
            network_id: 3,
            from: deployerAddressRopsten.toLowerCase(),
        },
        kovan: {
          provider: function() {
            return new HDWalletProvider(mnemonicKovan, infuraLinkKovan, addressIndex);
          },
          network_id: 42,
          from: deployerAddressKovan.toLowerCase(),
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
  // See <http://truffleframework.com/docs/advanced/configuration>
  // to customize your Truffle configuration!
};
