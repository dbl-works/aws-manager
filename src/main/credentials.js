const os = require("os");
const path = require("path");
const { readFile } = require('fs/promises');
const DEFAULT_CREDENTIALS_PATH = path.resolve(`${os.homedir()}/.aws/credentials`)

class Credentials {
  constructor() {
    this.parsedCredentials = {}
  }

  // 
  fetch() {
    readFile(DEFAULT_CREDENTIALS_PATH, "utf-8")
      .then((result) => {
        this.parse(result)
        mainWindow.webContents.send("credentials", this.parsedCredentials);
      })
      .catch((e) => {
        throw e;
      });
  }

  // @TODO: refactor
  parse(fileContent) {
    let lines = fileContent.trim().split('\n')

    for (let i = 0; i < lines.length; i += 3) {
      let profile = lines[i].replaceAll(/\[|\]/g, '')
      this.parsedCredentials[profile] = {}
      
      let [key_1, val_1] = lines[i + 1].split('=').map(v => v.trim())
      this.parsedCredentials[profile][key_1] = val_1

      let [key_2, val_2] = lines[i + 2].split('=').map(v => v.trim())
      this.parsedCredentials[profile][key_2] = val_2
    }
  }

  validate() {
    // @TODO
  }
}

module.exports = Credentials