'use strict';

var fs = require('fs');
var crypto = require("crypto");

var count = 0;
var walkPath = './solana_metadata';

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
					console.log(file);

					let rawdata = fs.readFileSync(file);
					let jsondata = JSON.parse(rawdata);

					var shasum = crypto.createHash("sha256");
					shasum.update(fs.readFileSync(`./images/${count}.png`));

					let variants = {};
					jsondata.attributes.forEach(element => {
						variants[element.trait_type] = element.value;
					});
					let metadata = {
						title: jsondata.name,
						description: jsondata.description,
						media: `${count}.png`,
						media_hash: shasum.digest("base64"),
						copies: 1,
						issued_at: null,
						expires_at: null,
						starts_at: null,
						updated_at: null,
						extra: JSON.stringify(variants),
						reference: null,
						reference_hash: null,
					};

					let data = JSON.stringify(metadata, null, '\t');
					fs.writeFileSync(`./near-metadata/${count}.json`, data);
					count++;
					next();
				}
			});
		})();
	});
};

// optional command line params
//      source for walk path
process.argv.forEach(function (val, index, array) {
	if (val.indexOf('source') !== -1) {
		walkPath = val.split('=')[1];
	}
});

console.log('-------------------------------------------------------------');
console.log('processing...');
console.log('-------------------------------------------------------------');

walk(walkPath, function (error) {
	if (error) {
		throw error;
	} else {
		console.log('-------------------------------------------------------------');
		console.log('finished.');
		console.log('-------------------------------------------------------------');
	}
});

// let rawdata = fs.readFileSync('student.json');
// let student = JSON.parse(rawdata);
// console.log(student);