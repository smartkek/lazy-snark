var Verifier = artifacts.require('Verifier');
var VerifierProxy = artifacts.require('VerifierProxy');
var Lazy = artifacts.require('Lazy');


module.exports = async function(deployer, network, accounts) {
	deployer.then(async() => {
		await deployer.deploy(Verifier)
		await deployer.deploy(VerifierProxy, Verifier.address)
		await deployer.deploy(Lazy, VerifierProxy.address)
	})
}
