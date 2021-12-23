export default class AWSCredentialsManager {
  constructor(credentials) {
    this.credentials = credentials
    this.currentProfile = this.defaultProfile()
    this.onProfileChange = null
    this.displayOptions()
  }

  displayOptions() {
    Object.keys(this.credentials).forEach((c, _) => {
      document.querySelector('#profile').insertAdjacentHTML(
        'beforeend',
        `<option value="${c}" ${ this.defaultProfile() == c ? 'selected' : '' }>${c}</option>`
      )
    })

    document.querySelector('#profile').addEventListener('change', e => {
      this.currentProfile = e.currentTarget.value
      this.onProfileChange?.call()
    })
  }

  defaultProfile() {
    return Object.keys(this.credentials).includes('default')
            ? 'default' : Object.keys(this.credentials)[0]
  }
}