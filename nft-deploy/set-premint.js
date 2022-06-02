const fs = require('fs');
const { exec } = require("child_process");

// The contract call to pre-mint / register tokens
// nft_register(& mut self, token_list: HashMap < String, TokenMetadata >)

var count = 0;
var batchIdx = 0;
var batchSize = 10;

var walkPath = './near_metadata';
var fileArray = new Array();

function sleep(ms) {
	return new Promise((resolve) => {
		setTimeout(resolve, ms);
	});
}

function randomize(arr) {
	for (let i = arr.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[arr[i], arr[j]] = [arr[j], arr[i]];
	}
}

async function batchTokens() {
	//for (let i = 0; i < fileArray.length; i += batchSize) {
	const i = batchIdx;
	batchIdx = i + batchSize
	files = fileArray.slice(i, batchIdx);
	//}
	let map = {};
	files.forEach(file => {
		let rawdata = fs.readFileSync(file);
		let jsondata = JSON.parse(rawdata);
		map[`AR#${count}`] = jsondata;
		count++
	});

	// console.log(`>> ${count}`);
	// console.log(JSON.stringify(map));
	// if (batchIdx < 1000) {
	// 	sleep(500);
	// 	batchTokens();
	// }

	exec(`near call $DEPLOY_ID nft_register '{"token_list":${JSON.stringify(map)},"amount":5}' --accountId $OWNER_ID --deposit 1`, (error, stdout, stderr) => {
		if (error) {
			console.log(`=> ${batchIdx} error:`);
			console.log(`${error.message}`);
		} else if (stderr) {
			console.log(`=> ${batchIdx} stderr:`);
			console.log(`${stderr}`);
		} else {
			console.log(`${batchIdx} => registered!`);
		}
		if (batchIdx < 1000) {
			sleep(500);
			batchTokens();
		}
	});
}

var walk = function (dir, done) {
	fs.readdir(dir, function (error, list) {
		if (error) {
			return done(error);
		}

		var i = 0;
		(function next() {
			var file = list[i++];
			if (!file) {
				return done(null);
			}

			file = dir + '/' + file;
			fs.stat(file, function (error, stat) {
				if (stat && stat.isDirectory()) {
					walk(file, function (error) {
						next();
					});
				} else {
					fileArray.push(file);
					count++;
					next();
				}
			});
		})();
	});
};

// optional command line params, sets source for walk path
// process.argv.forEach(function (val, index, array) {
// 	if (val.indexOf('source') !== -1) {
// 		walkPath = val.split('=')[1];
// 	}
// });

console.log('-------------------------------------------------------------');
console.log('gathering...');
walk(walkPath, function (error) {
	if (error) {
		throw error;
	}

	console.log('-------------------------------------------------------------');
	console.log('randomizing...');
	if (fileArray.length != 1000)
		console.log(`Error, expected 1k tokens`);
	randomize(fileArray);
	for (let i = 0; i < fileArray.length; i++) {
		console.log(`${i} => ${fileArray[i]}`);
	}

	console.log('-------------------------------------------------------------');
	console.log('registering...');
	// TODO Implement
	count = 0;
	batchIdx = 0;
	batchTokens();
});