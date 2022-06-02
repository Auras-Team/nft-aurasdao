'use strict';

const fs = require('fs');
const path = require('path');
const sharp = require('sharp');


var count = 0;
var walkPath = './images';

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

					let fileInfo = path.parse(file);
					sharp(file).resize({ height: 1000 }).toFile(`./v1/base/${fileInfo.name}.png`)
						.catch(function (err) {
							console.log(`Error occured on: t ${fileInfo.name}`);
						});
					sharp(file).resize({ height: 500 }).toFile(`./v1/small/${fileInfo.name}.png`)
						.catch(function (err) {
							console.log(`Error occured on: s ${fileInfo.name}`);
						});
					sharp(file).resize({ height: 250 }).toFile(`./v1/thumb/${fileInfo.name}.png`)
						.catch(function (err) {
							console.log(`Error occured on: t ${fileInfo.name}`);
						});

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
walk(walkPath, function (error) {
	if (error) {
		throw error;
	} else {
		console.log('-------------------------------------------------------------');
		console.log("finished.")
		console.log('-------------------------------------------------------------');
	}
});