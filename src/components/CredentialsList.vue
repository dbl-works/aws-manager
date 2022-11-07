<template>
  <div>
    <h2>{{ msg }}</h2>
    <p>{{ AWSCredentials }}</p>
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
      AWSCredentials: 'Unable to load AWS credentials file.',
    }
  },
  mounted() {
    this.loadAWSCredentials()
  },
  methods: {
    loadAWSCredentials() {
      invoke('load_credentials').then((body) => this.AWSCredentials = body)
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
