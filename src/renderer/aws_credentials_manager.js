export default class AWSCredentialsManager {
  constructor(credentials) {
    this.credentials = credentials
    this.onProfileChange = null
    this.displayOptions()
    this.addEditableProfiles()
    this.setupListeners()
  }

  onNewProfile() {
    document.querySelectorAll('.profiles-form input').forEach(input => {
      input.value = ''
    })
    document.querySelector('.save').innerText = 'Create'
  }

  setupListeners() {
    document.querySelector('.profiles .new-profile').removeEventListener('click', this.onNewProfile)
    document.querySelector('.profiles .new-profile').addEventListener('click', this.onNewProfile)

    document.querySelector('.save').removeEventListener('click', () => {
      this.updateOrCreateProfile()
    })
    document.querySelector('.save').addEventListener('click', () => {
      this.updateOrCreateProfile()
    })
  }

  displayOptions() {
    Object.keys(this.credentials).forEach((c, _) => {
      document.querySelector('#profile').insertAdjacentHTML(
        'beforeend',
        `<option value="${c}" ${ this.defaultProfile() == c ? 'selected' : '' }>${c}</option>`
      )
    })

    document.querySelector('#profile').addEventListener('change', e => {
      this.onProfileChange?.call()
    })
  }

  updateOrCreateProfile() {
    let profileIdentifier = document.querySelector('.profiles-form #profile-identifier').value
    let profileData = {}
    let profileDataKeys = ['name', 'aws_access_key_id', 'aws_secret_access_key', 'region', 'output']

    profileDataKeys.forEach((key, _) => {
      profileData[key] = document.querySelector('.profiles-form #' + key).value
    })

    if (Object.keys(this.credentials).includes(profileData['name']) && !profileIdentifier) {
      // TODO: show error, that you can't create user with the same profile name
      return
    }
   
    window.ipcRenderer.send(
      'updateOrCreateProfile',
      {
        profile_identifier: profileIdentifier,
        data: profileData
      }
    )
  }

  addEditableProfiles() {
    Object.keys(this.credentials).forEach((c, _) => {
      document.querySelector('.profiles-list').insertAdjacentHTML(
        'beforeend',
        `
        <div class="profile flex bg-gray-800 rounded-lg p-4 m-2 justify-between">
          <h5 class="name px-4 text-white w-full">${c}</h5>
          <button class="px-2 rounded bg-gray-100 bg-transparent text-gray-800 edit" data-profile-name="${c}">⚙️</button>
          <button class="px-2 rounded bg-gray-100 bg-transparent text-gray-800 remove" data-profile-name="${c}">🗑️</button>
        </div>
        `
      )
    })

    document.querySelectorAll('.profile .edit').forEach(btn => {
      btn.addEventListener('click', e => {
        let profileName = e.target.dataset['profileName']
        let profileData = this.credentials[profileName]

        document.querySelector('.profiles-form #name').value = profileName
        document.querySelector('.profiles-form #profile-identifier').value = profileName
        Object.keys(profileData).forEach((key, _) => {
          document.querySelector('.profiles-form #' + key).value = profileData[key]
        })

        document.querySelector('.save').innerText = 'Update'
      })
    })

    document.querySelectorAll('.profile .remove').forEach(btn => {
      btn.addEventListener('click', e => {
        let profileName = e.target.dataset['profileName']

        window.ipcRenderer.send(
          'removeProfile',
          { profile_identifier: profileName }
        )
      })
    })
  }

  defaultProfile() {
    return Object.keys(this.credentials).includes('default')
            ? 'default' : Object.keys(this.credentials)[0]
  }

  currentProfile() {
    return document.querySelector('#profile').value
  }
}