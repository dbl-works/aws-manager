const { RDSClient, DescribeDBInstancesCommand } = require("@aws-sdk/client-rds");
const shell = require('shelljs')
const { fromIni } = require("@aws-sdk/credential-provider-ini");

const REGION = "eu-central-1";

const listRDS = (profile) => {
  this.rds = new RDSClient({
    region: REGION,
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
// @TODO: make it use vanilla AWS-SDK methods 
const getAuthRDS = (db_config, profile) => {
  let cmd = `aws rds generate-db-auth-token --hostname "${db_config.hostname}" --port "${db_config.port}" --region eu-central-1 --username "${db_config.username}"`

  var child = shell.exec(cmd, { async: true });
  child.stdout.on('data', function(data) {
    mainWindow.webContents.send(
      'rdsGetAuthToken', data
    );
  });
}

module.exports.listRDS = listRDS
module.exports.getAuthRDS = getAuthRDS