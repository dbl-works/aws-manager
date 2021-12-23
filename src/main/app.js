const { ipcMain } = require("electron");
const Credentials = require('./credentials.js')

class App {
  constructor(mainWindow) {
    this.mainWindow = mainWindow;
    this.credentials = new Credentials()
    this.setupListeners()
  }

  setupListeners() {
    mainWindow.webContents.on("did-finish-load", () => {
      this.credentials.fetch()
    })
  }
}

module.exports = App;
