const { contextBridge, ipcRenderer, clipboard } = require('electron')

contextBridge.exposeInMainWorld('ipcRenderer', {
  send: (channel, data) => {
    ipcRenderer.send(channel, data)
  },
  receive: (channel, func) => {
    ipcRenderer.on(channel, (event, ...args) => func(...args))
  }
})

contextBridge.exposeInMainWorld('clipboard', {
  writeText: (token) => {
    clipboard.writeText(token)
  }
})
