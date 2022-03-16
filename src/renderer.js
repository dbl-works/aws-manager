import './css/index.css';
import './tabs.js'
import RDSManager from './renderer/rds_manager.js'
import AWSCredentialsManager from './renderer/aws_credentials_manager.js'

window.ipcRenderer.receive('credentials', (credentials) => {
  document.querySelector('#profile').innerHTML = ''
  document.querySelector('.profiles-list').innerHTML = ''

  let aws_cm = new AWSCredentialsManager(credentials)
  let rds_manager = new RDSManager(aws_cm)
  aws_cm.onProfileChange = () => {
    rds_manager.getRDSInstances()
  }
  let ipc_router = require('./renderer/rds_ipc_router.js')(rds_manager)
})

window.ipcRenderer.receive('no-credentials', () => {
  // TODO: Show some error
})

