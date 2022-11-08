<template>
  <div>
    <h2>{{ msg }}</h2>
    <div v-for="credential in AWSCredentials" :key="credential.profile_name">
      <span>Profile Name: {{ credential.profile_name }}</span>
      <br/>
      <span>Access Key ID: {{ credential.aws_access_key_id }}</span>
      <br/>
      <span>Secret Access Key: {{ credential.aws_secret_access_key }}</span>
      <br/>
      <span>Region: {{ credential.region }}</span>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri'

export default {
  props: {
    msg: String
  },
  data() {
    return {
      AWSCredentials: [{
        profile_name: 'default',
        aws_access_key_id: 'your key ID',
        aws_secret_access_key: 'your secret key',
        region: 'eu-central-1'
      }],
    }
  },
  mounted() {
    this.loadAWSCredentials()
  },
  methods: {
    loadAWSCredentials() {
      invoke('aws_credentials_index').then((body) => this.AWSCredentials = JSON.parse(body))
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
