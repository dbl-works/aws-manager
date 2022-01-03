export default class RDSManager {
  constructor(aws_cm) {
    this.aws_cm = aws_cm
    this.db_instances = {}
    this.getRDSInstances()
  }

  displayDatabases({ DBInstances }) {
    DBInstances.forEach((i, _) => {
      if (!i.DBName) return

      let [project, env] = i.DBName.split('_')
      this.db_instances[i.DBName] = {
        name: i.DBName,
        project: project,
        region: i.DBInstanceArn.split(':')[3],
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
        this.databaseTemplate(name, this.db_instances[name].username)
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

  databaseTemplate(name, username) {
    return `
      <tr>
        <td class="px-6 py-4 whitespace-nowrap">
          <div class="flex items-center">
            <div class="ml-4">
              <div class="text-sm font-medium text-gray-900">
                ${name}
              </div>
              <div class="text-sm text-gray-500">
                ${username}
              </div>
            </div>
          </div>
        </td>
        <td class="px-6 py-4 whitespace-nowrap">
          <button class="generate-password px-2 rounded bg-green-100 text-green-800" data-name="${name}">🔑 Generate Password</button>
        </td>
      </tr>
    `
  }

  showToken(token) {
    document.getElementById('password').innerText = token
  }

  getRDSInstances() {
    window.ipcRenderer.send('rdsList', { profile: this.aws_cm.currentProfile() })
  }
}
