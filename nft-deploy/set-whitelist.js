const fs = require('fs');
const { exec } = require("child_process");

// The contract call to whitelist / allow minting tokens
// nft_allow_minting(&mut self, account_id: AccountId, amount: u32)

// Listed 120 accounts (4 fails)
// Pre Used:  300459
// Pre Cost:  3004590000000000000000000
// Post Used: 309772
// Post Cost: 3097720000000000000000000

// Not a csv file! But a list of accounts, one per line.
// Copy a colunm from the sheet in to a file with name below.

let file = './whitelist.csv';

function echo(key) {
	exec(`echo ${key}`, (error, stdout, stderr) => {
		if (error) {
			console.log(`error: ${error.message}`);
			return;
		}
		if (stderr) {
			console.log(`stderr: ${stderr}`);
			return;
		}
		console.log(`${key}: ${stdout}`);
	});
}

function sleep(ms) {
	return new Promise((resolve) => {
		setTimeout(resolve, ms);
	});
}

let idx = 0;
let list = new Array();
function nextAccount() {
	idx++;
	sleep(500);
	if (idx < list.length) {
		processAccount(list[idx]);
	}
}

async function processAccount(account) {
	if (account.len() <= 5 && "0x" == account.substring(0, 2)) {
		console.log(`${idx} => Skiping: ${account}`);
		nextAccount();
		return;
	}

	exec(`near call $DEPLOY_ID nft_allow_minting '{"account_id":"${account.toLowerCase()}","amount":5}' --accountId $OWNER_ID --depositYocto 1`, (error, stdout, stderr) => {
		if (error) {
			console.log(`=> ${idx} => ${account}`);
			console.log(`error: ${error.message}`);
		} else if (stderr) {
			console.log(`=> ${idx} => ${account}`);
			console.log(`stderr: ${stderr}`);
		} else {
			console.log(`${idx} => Listed: ${account}`);
		}
		nextAccount();
	});
}

echo("$OWNER_ID");
echo("$DEPLOY_ID");


let fileData = fs.readFileSync(file, 'utf-8');
fileData.split(/\r?\n/).forEach(line => {
	list.push(line)
});
processAccount(list[0]);