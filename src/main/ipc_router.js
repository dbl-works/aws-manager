const { ipcMain, BrowserWindow } = require("electron");
const { listRDS, getAuthRDS } = require("./rds.js");
const Credentials = require('./credentials.js')

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

ipcMain.on("updateOrCreateProfile", (event, args) => {
  let credentials = new Credentials()

  if (args.profile_identifier) {
    credentials.update(args.profile_identifier, args.data)
  } else {
    credentials.create(args.data)
  }
})

ipcMain.on("removeProfile", (event, args) => {
  let credentials = new Credentials()

  credentials.remove(args.profile_identifier)
})
