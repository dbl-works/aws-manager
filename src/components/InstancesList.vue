<template>
  <ul role="list" class="divide-y divide-gray-100">
    <li v-for="instance in RDSInstances" :key="instance.name" class="flex space-between gap-1.5 py-5">
      <div class="flex w-96">
        <div class="min-w-0 flex-auto">
          <p class="text-sm font-semibold leading-6 text-white-900">{{ instance.name }}</p>
          <p class="mt-1 truncate text-xs leading-5 text-white-500">{{ instance.endpoint }}</p>
          <div>
            <p class="mt-1 truncate text-xs leading-5 text-white-500">{{ instance.instance_class }}</p>
            <div class="flex items-row gap-x-1.5 justify-items-center">
              <p class="mt-1 truncate text-xs leading-5 text-white-500">{{ instance.engine }}</p>
              <div v-if="instance.status == 'available'" class="mt-1 text-xs leading-5 text-white-500">
                <div class="flex-none rounded-full bg-emerald-500/20 p-1">
                  <div class="h-1.5 w-1.5 rounded-full bg-emerald-500" />
                </div>
              </div>
              <div v-else class="mt-1 flex items-center gap-x-1.5">
                <div class="flex-none rounded-full bg-red-500/20 p-1">
                  <div class="h-1.5 w-1.5 rounded-full bg-red-500" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div class="flex-1">
        <code class="flex items-end flex-col h-40 block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="password will appear here...">
          <button class="bg-black-50 rounded-lg" @click="copyToClipboard(instance.password)">
            <ClipboardIcon class="h-5 w-5 text-gray-400"/>
          </button>
          {{ instance.password }}
        </code>
      </div>
      <button class="flex-end w-32 flex flex-col items-center justify-center" @click="generatePassword(instance)">
        <LockClosedIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
      </button>
    </li>
  </ul>
</template>

<script>
  import { invoke } from '@tauri-apps/api';
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
      },
      refreshDatabases(profile) {
        invoke('set_aws_profile', { profileName: profile });
        invoke('aws_rds_index').then((body) => {
          try {
            this.RDSInstances = JSON.parse(body)
          } catch (e) {
            console.log(e);
          }
        });
      },
      generatePassword(instance) {
        invoke('generate_password', {
          hostname: instance.endpoint,
          port: instance.port,
          username: instance.username,
        }).then((body) => {
          instance.password = body;
          this.copyToClipboard(instance.password);
        });
      },
    }
  }
</script>
