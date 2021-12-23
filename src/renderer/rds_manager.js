export default class RDSManager {
  constructor(aws_cm) {
    this.aws_cm = aws_cm
    this.db_instances = {}
    this.getRDSInstances()
  }

  displayDatabases({ DBInstances }) {
    DBInstances.forEach((i, _) => {
      let [project, env] = i.DBName.split('_')
      this.db_instances[i.DBName] = {
        name: i.DBName,
        project: project,
        env: env,
        username: `${project}_${env}_readonly`,
        hostname: i.Endpoint.Address,
        port: i.Endpoint.Port
      }
    })
    this.showInstances()
  }

  showInstances() {
    document.getElementById('databases').innerHTML = ''

    Object.keys(this.db_instances).forEach((name, _) => {
      document.getElementById('databases').insertAdjacentHTML(
        'beforeend',
        `
          <p>${name}</p>
          <button class="generate-password" data-name="${name}">🔑 Generate</button>
        `
      )
    })

    document.querySelectorAll('.generate-password').forEach((el, _) => {
      el.addEventListener('click', (e) => {
        let db = this.db_instances[e.currentTarget.dataset.name]
        window.ipcRenderer.send(
          'rdsGetAuthToken',
          {
            db: db,
            profile: this.aws_cm.defaultProfile()
          }
        )
      })
    })
  }

  showToken(token) {
    document.getElementById('password').innerText = token
  }

  getRDSInstances() {
    window.ipcRenderer.send('rdsList', { profile: this.aws_cm.defaultProfile() })
  }
}
