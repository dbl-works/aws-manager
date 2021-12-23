module.exports = function(rds_manager) {
  window.ipcRenderer.receive('rdsList', (msg) => {
    rds_manager.displayDatabases(msg)
  })

  window.ipcRenderer.receive('rdsGetAuthToken', (msg) => {
    rds_manager.showToken(msg)
  })
}