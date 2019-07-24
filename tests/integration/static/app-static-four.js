app.port = 3004;

app.static("/", "./files", { index: false, cleanUrls: true});
