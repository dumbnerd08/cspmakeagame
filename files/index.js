const express = require("express")
const app = express();
const path = require('path');

app.get("/space.hdr", function (req, res) {
	const options = {
		root: path.join(__dirname)
	};

	const fileName = "space.hdr";
	res.sendFile(fileName, options, function (err) {
		if (err) {
			console.log("error");
		} else {
			console.log("sent");
		}
	});
});

app.get("/iss.obj", function (req, res) {
	const options = {
		root: path.join(__dirname)
	};

	const fileName = "iss.obj";
	res.sendFile(fileName, options, function (err) {
		if (err) {
			console.log("error");
		} else {
			console.log("sent");
		}
	});
});

app.listen(8080, function (err) {
	if (err) console.error(err);
	console.log("listening on port 8080");
});
