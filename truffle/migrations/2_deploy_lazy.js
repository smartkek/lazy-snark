var Verifier = artifacts.require('./Verifier.sol');
var VerifierProxy = artifacts.require('./VerifierProxy.sol');
var Lazy = artifacts.require('./Lazy.sol');

const proof1 = [["0x24d858a8ffc1766e7cccf95643f1339cb10978a5b06a6f8abad82782a3ab3efd","0x2d0b5206a856f75a93284728f7cfa61e998868b632b11886a439add8e6150f3e"],[["0x13e6a883986efe7bb0c6e99b00c157d31ea8479283ac6c4934e42368da31e6d8","0x0718d8dfecf7726df8a27092bac85d58767230b33a66dd8c69ca54f576361fc1"],["0x160f03315ee7766d576122244d12bcabd24ee3c7d33e1cb05d838337a0fd0dfb","0x2ea4c7c18ed1b8f3544a4fb3c4d95efa5543182406fef71bfda8760e407a05c7"]],["0x1111708b97dac3f087a3d8e13cfd513d7a82c619d39e9148080b600b7dae210e","0x2a6e76f3003e65afefae71d3fe18ba26a3940e81463bf94615478cdf5daa2249"]];
const input1 = [["0x00000000000000000000000000000000c6481e22c5ff4164af680b8cfaa5e8ed","0x000000000000000000000000000000003120eeff89c4f307c4a6faaae059ce10","0x000000000000000000000000000000005b6d7d198c48c17c9540d29275a04662","0x00000000000000000000000000000000f7a9aa434629a33c84eec3e16e196f27","0x0000000000000000000000000000000000000000000000000000000000000001"]];

module.exports = function(deployer, network, accounts) {
	console.log("kuku");
	deployer.then(async() => {
		await deployer.deploy(Verifier);
		await deployer.deploy(VerifierProxy, Verifier.address);
		console.log(Verifier.address);
		var contract = await deployer.deploy(Lazy, VerifierProxy.address);
		console.log(VerifierProxy.address);
		await contract.submit(input1, proof1);
		await contract.submit(input1, proof1);
		await contract.submit(input1, proof1);
		await contract.submit(input1, proof1);
		await contract.submit([[1, 2, 3, 4, 5]], [[1, 2], [[3, 4], [5, 6]], [7, 8]]);

	})
}
