<template>
  <div class="min-h-screen bg-gray-800 text-white">
    <img alt="Vue logo" src="./assets/logo.png" height="50" width="50">
    <CredentialsList msg="Credentials List"/>
  </div>
</template>

<script>
import CredentialsList from './components/CredentialsList.vue'

export default {
  name: 'App',
  components: {
    CredentialsList
  }
}
</script>
