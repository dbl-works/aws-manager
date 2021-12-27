const { ipcMain, BrowserWindow } = require("electron");
const { listRDS, getAuthRDS } = require("./rds.js");

ipcMain.on("rdsList", (event, args) => {
  listRDS(args.profile);
});

ipcMain.on("rdsGetAuthToken", (event, args) => {
  getAuthRDS(
    {
      hostname: args.db.hostname,
      port: args.db.port,
      username: args.db.username,
      region: args.db.region,
      credentials: args.db.credentials,
    },
    args.profile
  );
});
