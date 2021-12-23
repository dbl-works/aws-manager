import './css/index.css';
import RDSManager from './renderer/rds_manager.js'
import AWSCredentialsManager from './renderer/aws_credentials_manager.js'

window.ipcRenderer.receive('credentials', (credentials) => {
  let aws_cm = new AWSCredentialsManager(credentials)
  let rds_manager = new RDSManager(aws_cm)
  aws_cm.onProfileChange = () => {
    rds_manager.getRDSInstances()
  }
  let ipc_router = require('./renderer/rds_ipc_router.js')(rds_manager)
})

