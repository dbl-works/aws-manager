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

  parse(fileContent) {
    let lines = fileContent.trim().split('\n')

    let currentProfile = ''
    for (let i = 0; i < lines.length; i++) {
      if (lines[i].includes('[') && lines[i].includes(']')) {
        let profile = lines[i].replaceAll(/\[|\]/g, '')
        currentProfile = profile
        this.parsedCredentials[currentProfile] = {}
      }

      if (lines[i].includes('aws_access_key_id') || lines[i].includes('aws_secret_access_key')) {
        let [key, val] = lines[i].split('=').map(v => v.trim())
        this.parsedCredentials[currentProfile][key] = val
      }
    }
  }

  validate() {
    // @TODO
  }
}

module.exports = Credentials