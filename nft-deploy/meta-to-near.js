'use strict';

const TOKEN_IMG_URL = "cdn.aurasdao.com/tokens/v1/base"

const fs = require('fs');
const path = require('path');
const crypto = require("crypto");

var count = 0;
var walkPath = './solana_metadata';

let check_id = new Array(1000);
let check_img = new Array(1000);

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

					let image_name = '--'
					if (count == 0) {
						image_name = `0.png`;
					} else {
						image_name = jsondata.image;
					}

					let nr = parseInt(jsondata.name.substring(5));
					if (nr == 1000) {
						check_id[0] = true;
					} else {
						check_id[nr] = true;
					}

					nr = parseInt(path.parse(image_name).name);
					check_img[nr] = true;

					var shasum = crypto.createHash("sha256");
					shasum.update(fs.readFileSync(`./images/${image_name}`));

					let variants = {};
					jsondata.attributes.forEach(element => {
						variants[element.trait_type] = element.value;
					});
					let metadata = {
						title: jsondata.name,
						media: `${TOKEN_IMG_URL}/${jsondata.image}`,
						media_hash: shasum.digest("base64"),
						attributes: JSON.stringify(variants),
					};

					let data = JSON.stringify(metadata, null, '\t');
					fs.writeFileSync(`./near_metadata/${count}.json`, data);
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
console.log('processing...');
console.log('-------------------------------------------------------------');

for (let i = 0; i < check_img.length; i++) {
	check_id[i] = false;
	check_img[i] = false;
}

walk(walkPath, function (error) {
	if (error) {
		throw error;
	} else {
		console.log('-------------------------------------------------------------');
		console.log('done.');
		console.log('-------------------------------------------------------------');
		console.log("verifying...")
		console.log('-------------------------------------------------------------');
		for (let i = 0; i < check_id.length; i++) {
			if (!check_id[i]) console.log(`Unused id: ${i}`);
		}
		for (let i = 0; i < check_img.length; i++) {
			if (!check_img[i]) console.log(`Unused Image: ${i}`);
		}
		console.log('-------------------------------------------------------------');
		console.log("finished.")
		console.log('-------------------------------------------------------------');
	}
});
