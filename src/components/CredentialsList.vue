<template>
  <div class="p-6">
    <h1 class="text-3xl font-bold mb-4">AWS RDS Instances</h1>
    <div class="mb-4">
      <label for="profile" class="block mb-2">AWS Profile</label>
      <select id="profile" v-model="selectedProfile" @change="switchProfile" class="w-full p-2 bg-gray-700 rounded">
        <option v-for="credential in AWSCredentials" :key="credential.profile_name" :value="credential.profile_name">{{ credential.profile_name }}</option>
      </select>
      <div>
        <div class="relative mt-2">
          <button type="button" class="relative w-full cursor-default rounded-md bg-white py-1.5 pl-3 pr-10 text-left text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 sm:text-sm sm:leading-6" aria-haspopup="listbox" aria-expanded="true" aria-labelledby="listbox-label">
            <span class="flex items-center">
              <span class="ml-3 block truncate">{{ selectedProfile.profile_name }}</span>
            </span>
            <span class="pointer-events-none absolute inset-y-0 right-0 ml-3 flex items-center pr-2">
              <svg class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                <path fill-rule="evenodd" d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z" clip-rule="evenodd" />
              </svg>
            </span>
          </button>
          <ul class="absolute z-10 mt-1 max-h-56 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm" tabindex="-1" role="listbox" aria-labelledby="listbox-label" aria-activedescendant="listbox-option-3">
            <li v-for="instance in RDSInstances" :key="instance.profile_name" class="text-gray-900 relative cursor-default select-none py-2 pl-3 pr-9" id="listbox-option-0" role="option" @click="refreshDatabases">
              <div class="flex items-center">
                <span class="font-normal ml-3 block truncate">{{ instance.profile_name }}</span>
              </div>
              <span v-if="selectedProfile.profile_name === instance.profile_name" class="text-indigo-600 absolute inset-y-0 right-0 flex items-center pr-4">
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                  <path fill-rule="evenodd" d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z" clip-rule="evenodd" />
                </svg>
              </span>
            </li>
          </ul>
        </div>
      </div>
    </div>
    <div v-if="loading" class="text-center py-4">
      <div class="loader"></div>
    </div>
    <div v-else>
      <div v-for="instance in RDSInstances" :key="instance.identifier" class="bg-gray-700 rounded p-4 mb-4">
        <h2 class="text-xl font-bold">{{ instance.identifier }}</h2>
        <p>Engine: {{ instance.engine }}</p>
        <p>Status: {{ instance.status }}</p>
        <p>Endpoint: {{ instance.endpoint }}</p>
        <button @click="generatePassword(instance.identifier)" class="mt-2 px-4 py-2 bg-green-500 hover:bg-green-600 rounded">Generate Password</button>
      </div>
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
      RDSInstances: [{
        identifier: 'identifier',
        engine: 'engine',
        status: 'status',
        endpoint: 'endpoint',
      }],
      selectedProfile: 'default', // default selected profile
    }
  },
  mounted() {
    this.loadAWSCredentials()
  },
  methods: {
    loadAWSCredentials() {
      invoke('aws_credentials_index').then((body) => {
        this.AWSCredentials = JSON.parse(body);
        // Refresh the databases when the credentials are loaded
        this.refreshDatabases();
      })
    },
    refreshDatabases() {
      invoke('set_aws_profile', { profileName: this.selectedProfile });
      invoke('aws_rds_index').then((body) => {
        try {
          this.RDSInstances = JSON.parse(body)
        } catch (e) {
          console.log(e);
        }
      });
    }
  }
}
</script>

<style scoped>
.loader {
  border: 16px solid #f3f3f3;
  border-top: 16px solid #3498db;
  border-radius: 50%;
  width: 120px;
  height: 120px;
  animation: spin 2s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
</style>
