const { RDSClient, DescribeDBInstancesCommand } = require("@aws-sdk/client-rds");
const { fromIni } = require("@aws-sdk/credential-provider-ini");
const getAuthToken = require('./signer.js')

const listRDS = (profile) => {
  this.rds = new RDSClient({
    credentials: fromIni({ profile })
  });

  const command = new DescribeDBInstancesCommand({});
  this.rds.send(command).then(result => {
    mainWindow.webContents.send(
      'rdsList',
      result
    );
  })
}

const getAuthRDS = (db_config, profile) => {
  getAuthToken(
    {
      ...db_config,
      credentials: fromIni({ profile })
    }
  ).then(token => {
    mainWindow.webContents.send('rdsGetAuthToken', token);
  })
}

module.exports.listRDS = listRDS
module.exports.getAuthRDS = getAuthRDS