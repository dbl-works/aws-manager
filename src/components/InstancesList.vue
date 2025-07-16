<template>
  <div class="space-y-6">
    <div v-if="RDSInstances.length === 0" class="text-center py-12">
      <div class="mx-auto h-12 w-12 text-gray-500">
        <svg fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4" />
        </svg>
      </div>
      <h3 class="mt-4 text-lg font-medium text-gray-400">No RDS instances found</h3>
      <p class="mt-2 text-gray-500">Select an AWS profile to load your RDS instances.</p>
    </div>

    <div v-else class="grid gap-6">
      <div
        v-for="instance in RDSInstances"
        :key="instance.name"
        class="group relative bg-gradient-to-r from-gray-800/50 to-gray-800/30 rounded-2xl border border-gray-700/50 p-6 hover:border-gray-600/50 transition-all duration-300 backdrop-blur-sm"
      >
        <!-- Instance Header -->
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1">
            <div class="flex items-center space-x-3 mb-2">
              <h3 class="text-lg font-semibold text-white">{{ instance.name }}</h3>
              <div class="flex items-center space-x-2">
                <div v-if="instance.status == 'available'" class="flex items-center space-x-1">
                  <div class="h-2 w-2 rounded-full bg-emerald-400 animate-pulse"></div>
                  <span class="text-xs font-medium text-emerald-400">Available</span>
                </div>
                <div v-else class="flex items-center space-x-1">
                  <div class="h-2 w-2 rounded-full bg-red-400"></div>
                  <span class="text-xs font-medium text-red-400">{{ instance.status }}</span>
                </div>
              </div>
            </div>

            <div class="space-y-1">
              <p class="text-sm text-gray-300 font-mono bg-gray-900/50 rounded-lg px-3 py-1 inline-block">
                {{ instance.endpoint }}
              </p>
              <div class="flex items-center space-x-4 text-xs text-gray-400">
                <span class="bg-gray-700/50 rounded-full px-2 py-1">{{ instance.instance_class }}</span>
                <span class="bg-gray-700/50 rounded-full px-2 py-1">{{ instance.engine }}</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Password Section -->
        <div class="grid grid-cols-1 lg:grid-cols-4 gap-4">
          <!-- Password Display -->
          <div class="lg:col-span-3">
            <label class="block text-sm font-medium text-gray-300 mb-2">Generated Password</label>
            <div class="relative">
              <div class="bg-gray-900/80 border border-gray-700/50 rounded-xl p-4 font-mono text-sm text-gray-300 min-h-[60px] flex items-center justify-between">
                <span v-if="instance.password" class="break-all">{{ instance.password }}</span>
                <span v-else class="text-gray-500 italic">Click generate to create a temporary password</span>

                <button
                  v-if="instance.password"
                  @click="copyToClipboard(instance.password)"
                  class="ml-3 p-2 rounded-lg bg-gray-800 hover:bg-gray-700 text-gray-400 hover:text-white transition-colors duration-200 group"
                  title="Copy to clipboard"
                >
                  <ClipboardIcon class="h-4 w-4"/>
                </button>
              </div>
            </div>
          </div>

          <!-- Generate Button -->
          <div class="lg:col-span-1">
            <label class="block text-sm font-medium text-gray-300 mb-2">Actions</label>
            <button
              @click="generatePassword(instance)"
              :disabled="!selectedProfile"
              class="w-full h-[60px] bg-gradient-to-r from-blue-600 to-blue-500 hover:from-blue-500 hover:to-blue-400 disabled:from-gray-700 disabled:to-gray-600 disabled:cursor-not-allowed text-white font-medium rounded-xl transition-all duration-200 flex items-center justify-center space-x-2 group"
            >
              <LockClosedIcon class="h-5 w-5"/>
              <span>Generate</span>
            </button>
          </div>
        </div>

        <!-- Loading State -->
        <div v-if="instance.loading" class="absolute inset-0 bg-gray-800/50 rounded-2xl flex items-center justify-center backdrop-blur-sm">
          <div class="flex items-center space-x-3 text-white">
            <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500"></div>
            <span>Generating password...</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
  import { invoke } from '@tauri-apps/api/core';
  import { LockClosedIcon, ClipboardIcon } from '@heroicons/vue/20/solid';

  export default {
    props: ['selectedProfile'],
    components: {
      ClipboardIcon,
      LockClosedIcon,
    },
    data() {
      return {
        RDSInstances: [],
        password: '',
      }
    },
    watch: {
      selectedProfile(newProfile) {
        this.refreshDatabases(newProfile)
      },
    },
    methods: {
      copyToClipboard(text) {
        navigator.clipboard.writeText(text);
        // You could add a toast notification here
      },
      refreshDatabases(profile) {
        this.RDSInstances = [];
        if (!profile) return;

        try {
          invoke('set_aws_profile', { profileName: profile });
          invoke('aws_rds_index').then((body) => {
            try {
              // Check if body is a string that starts with "Error" or similar
              if (typeof body === 'string' && (body.startsWith('Error') || body.startsWith('error'))) {
                throw new Error(body);
              }
              this.RDSInstances = JSON.parse(body).map(instance => ({
                ...instance,
                loading: false,
                password: null
              }));
            } catch (e) {
              console.log('Error parsing RDS instances:', e);
              // Show mock data for browser testing or when API fails
              if (typeof window !== 'undefined' && !window.__TAURI__) {
                this.RDSInstances = [
                  {
                    name: 'Demo RDS Instance',
                    endpoint: 'demo-instance.amazonaws.com',
                    instance_class: 'db.t3.medium',
                    engine: 'postgres',
                    status: 'available',
                    port: 5432,
                    username: 'postgres',
                    loading: false,
                    password: null
                  }
                ];
              }
            }
          }).catch((error) => {
            console.error('Error fetching RDS instances:', error);
            // Show mock data for browser testing
            if (typeof window !== 'undefined' && !window.__TAURI__) {
              this.RDSInstances = [
                {
                  name: 'Demo RDS Instance',
                  endpoint: 'demo-instance.amazonaws.com',
                  instance_class: 'db.t3.medium',
                  engine: 'postgres',
                  status: 'available',
                  port: 5432,
                  username: 'postgres',
                  loading: false,
                  password: null
                }
              ];
            }
          });
        } catch (error) {
          console.error('Tauri API not available:', error);
          // Show mock data for browser testing
          if (typeof window !== 'undefined' && !window.__TAURI__) {
            this.RDSInstances = [
              {
                name: 'Demo RDS Instance',
                endpoint: 'demo-instance.amazonaws.com',
                instance_class: 'db.t3.medium',
                engine: 'postgres',
                status: 'available',
                port: 5432,
                username: 'postgres',
                loading: false,
                password: null
              }
            ];
          }
        }
      },
      generatePassword(instance) {
        instance.loading = true;
        try {
          invoke('generate_password', {
            hostname: instance.endpoint,
            port: instance.port,
            username: instance.username,
            profile: this.selectedProfile,
          }).then((body) => {
            instance.password = body;
            instance.loading = false;
            this.copyToClipboard(instance.password);
          }).catch((error) => {
            console.error('Error generating password:', error);
            instance.loading = false;
            // Mock password for browser testing
            if (typeof window !== 'undefined' && !window.__TAURI__) {
              instance.password = 'demo_password_' + Math.random().toString(36).substr(2, 9);
              this.copyToClipboard(instance.password);
            }
          });
        } catch (error) {
          console.error('Tauri API not available:', error);
          instance.loading = false;
          // Mock password for browser testing
          if (typeof window !== 'undefined' && !window.__TAURI__) {
            instance.password = 'demo_password_' + Math.random().toString(36).substr(2, 9);
            this.copyToClipboard(instance.password);
          }
        }
      },
    }
  }
</script>
