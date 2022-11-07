<template>
  <img alt="Vue logo" src="./assets/logo.png" height="50" width="50">
  <CredentialsList msg="Credentials List"/>
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

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
