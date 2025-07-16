<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-900 via-gray-800 to-gray-900">
    <!-- Header -->
    <header class="border-b border-gray-700/50 bg-gray-900/50 backdrop-blur-sm">
      <div class="max-w-7xl mx-auto px-6 py-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center space-x-4">
            <div class="relative">
              <img class="h-10 w-10 rounded-lg shadow-lg" src="./assets/logo.png">
              <div class="absolute -inset-1 bg-gradient-to-r from-blue-600 to-purple-600 rounded-lg opacity-20 blur"></div>
            </div>
            <div>
              <h1 class="text-2xl font-bold text-white tracking-tight">
                AWS Manager
              </h1>
              <p class="text-sm text-gray-400 font-mono">
                v{{ this.version }}
              </p>
            </div>
          </div>
          <ProfilesList @profile-changed="updateSelectedProfile"/>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="max-w-7xl mx-auto px-6 py-8">
      <div class="mb-8">
        <h2 class="text-xl font-semibold text-white mb-2">RDS Instances</h2>
        <p class="text-gray-400">Manage your AWS RDS database instances and generate temporary passwords</p>
      </div>
      <InstancesList :selectedProfile="selectedProfile"/>
    </main>
  </div>
</template>

<script>
  import { getVersion } from '@tauri-apps/api/app';
  import InstancesList from './components/InstancesList.vue';
  import ProfilesList from './components/ProfilesList.vue';

  export default {
    name: 'App',
    data() {
      return {
        selectedProfile: null,
        version: null,
      };
    },
    mounted() {
      this.appVersion();
    },
    methods: {
      updateSelectedProfile(newProfile) {
        this.selectedProfile = newProfile;
      },
      async appVersion() {
        try {
          this.version = await getVersion();
        } catch (error) {
          console.error('Tauri API not available:', error);
          // Fallback version for browser testing
          if (typeof window !== 'undefined' && !window.__TAURI__) {
            this.version = 'dev-browser';
          }
        }
      },
    },
    components: {
      InstancesList,
      ProfilesList
    },
  }
</script>
