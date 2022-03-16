const os = require("os");
const path = require("path");
const { appendFile } = require('fs/promises');
const { readFileSync, writeFileSync } = require('fs');
const DEFAULT_CREDENTIALS_PATH = path.resolve(`${os.homedir()}/.aws/credentials`)

class Credentials {
  constructor() {
    this.parsedCredentials = {}
  }

  // 
  fetch() {
    try {
      let result = readFileSync(DEFAULT_CREDENTIALS_PATH, "utf-8")
      this.parse(result)
    } catch {
      mainWindow.webContents.send("no-credentials");
    }
  }

  sync() {
    mainWindow.webContents.send("credentials", this.parsedCredentials);
  }

  parse(fileContent) {
    this.parsedCredentials = {}

    let lines = fileContent.trim().split('\n')

    let currentProfile = ''
    for (let i = 0; i < lines.length; i++) {
      if (lines[i].includes('[') && lines[i].includes(']')) {
        let profile = lines[i].replaceAll(/\[|\]/g, '')
        currentProfile = profile
        this.parsedCredentials[currentProfile] = {}
      }

      if (lines[i].match(/(aws_access_key_id|aws_secret_access_key|region|output)/)) {
        let [key, val] = lines[i].split('=').map(v => v.trim())
        this.parsedCredentials[currentProfile][key] = val
      }

      this.parsedCredentials[currentProfile]['name'] = currentProfile
    }
  }

  async create(profileData) {
    let res = await appendFile(DEFAULT_CREDENTIALS_PATH, "\n\n" + this.buildProfileLines(profileData))

    this.fetch()
    this.sync()
  }

  async update(profileIdentifier, profileData) {
    // Sync with current version of credentials
    this.parsedCredentials = {}
    this.fetch()

    if (!this.parsedCredentials[profileIdentifier]) return

    // Update changed profile
    this.parsedCredentials[profileIdentifier].name = profileData['name']
    this.parsedCredentials[profileIdentifier].aws_access_key_id = profileData['aws_access_key_id']
    this.parsedCredentials[profileIdentifier].aws_secret_access_key = profileData['aws_secret_access_key']
    this.parsedCredentials[profileIdentifier].region = profileData['region']
    this.parsedCredentials[profileIdentifier].output = profileData['output']

    // Build new version of credentials file
    let newCredentials = Object.keys(this.parsedCredentials).map(profile => {
      return this.buildProfileLines(this.parsedCredentials[profile])
    }).join('\n\n')

    writeFileSync(DEFAULT_CREDENTIALS_PATH, newCredentials)

    this.fetch()
    this.sync()
  }

  async remove(profileIdentifier) {
    // Sync with current version of credentials
    this.parsedCredentials = {}
    this.fetch()

    if (!this.parsedCredentials[profileIdentifier]) return

    // Remove profile
    delete this.parsedCredentials[profileIdentifier]

    // Build new version of credentials file
    let newCredentials = Object.keys(this.parsedCredentials).map(profile => {
      return this.buildProfileLines(this.parsedCredentials[profile])
    }).join('\n\n')

    writeFileSync(DEFAULT_CREDENTIALS_PATH, newCredentials)

    this.fetch()
    this.sync()
  }

  buildProfileLines(profileData) {
    let region = profileData['region'] ? `region = ${profileData['region']}` : ''
    let output = profileData['output'] ? `output = ${profileData['output']}` : ''

    return [
      `[${profileData['name']}]`,
      `aws_access_key_id = ${profileData['aws_access_key_id']}`,
      `aws_secret_access_key = ${profileData['aws_secret_access_key']}`,
      `${region}`,
      `${output}`
    ].join('\n')
  }
}

module.exports = Credentials