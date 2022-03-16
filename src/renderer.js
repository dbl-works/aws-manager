import './css/index.css';
import './tabs.js'
import RDSManager from './renderer/rds_manager.js'
import AWSCredentialsManager from './renderer/aws_credentials_manager.js'

window.ipcRenderer.receive('credentials', (credentials) => {
  document.querySelector('#profile').innerHTML = ''
  document.querySelector('.profiles-list').innerHTML = ''

  if (!window.aws_cm) {
    window.aws_cm = new AWSCredentialsManager(credentials)
    aws_cm.onProfileChange = () => {
      rds_manager.getRDSInstances()
    }
  } else {
    window.aws_cm.credentials = credentials
    window.aws_cm.displayOptions()
    window.aws_cm.addEditableProfiles()
    window.rds_manager.getRDSInstances()
  }

  if (!window.rds_manager) {
    window.rds_manager ||= new RDSManager(aws_cm)
    let ipc_router = require('./renderer/rds_ipc_router.js')(rds_manager)
  }
})

window.ipcRenderer.receive('no-credentials', () => {
  // TODO: Show some error
  return
})

