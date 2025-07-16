<template>
  <Listbox v-model="selectedProfile" @update:model-value="onProfileChange">
    <div class="relative">
      <ListboxButton
        class="relative w-64 cursor-pointer rounded-xl bg-gray-800/80 border border-gray-700/50 py-3 px-4 text-left shadow-lg hover:bg-gray-800 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-900 transition-all duration-200"
      >
        <span class="flex items-center">
          <span class="block truncate text-white font-medium">{{ selectedProfile }}</span>
        </span>
        <span class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3">
          <ChevronUpDownIcon
            class="h-5 w-5 text-gray-400"
            aria-hidden="true"
          />
        </span>
      </ListboxButton>

      <transition
        leave-active-class="transition duration-100 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <ListboxOptions
          class="absolute z-[9999] mt-2 max-h-60 w-full min-w-max overflow-auto rounded-xl bg-gray-800 border border-gray-700/50 py-2 text-base shadow-2xl ring-1 ring-black ring-opacity-5 focus:outline-none backdrop-blur-sm"
        >
          <ListboxOption
            v-slot="{ active, selected }"
            v-for="profile in profiles"
            :key="profile.profile_name"
            :value="profile.profile_name"
            as="template"
          >
            <li
              :class="[
                active ? 'bg-blue-600/20 text-blue-300' : 'text-gray-300',
                'relative cursor-pointer select-none py-3 pl-10 pr-4 transition-colors duration-150',
              ]"
            >
              <span
                :class="[
                  selected ? 'font-semibold text-white' : 'font-normal',
                  'block truncate',
                ]"
                >{{ profile.profile_name }}</span
              >
              <span
                v-if="selected"
                class="absolute inset-y-0 left-0 flex items-center pl-3 text-blue-400"
              >
                <CheckIcon class="h-5 w-5" aria-hidden="true" />
              </span>
            </li>
          </ListboxOption>
        </ListboxOptions>
      </transition>
    </div>
  </Listbox>
</template>

<script>
  import { invoke } from '@tauri-apps/api/core';
  import { CheckIcon, ChevronUpDownIcon } from '@heroicons/vue/20/solid'
  import {
    Listbox,
    ListboxButton,
    ListboxOptions,
    ListboxOption,
  } from '@headlessui/vue';

  export default {
    data() {
      return {
        selectedProfile: 'default',
        profiles: [],
      }
    },
    components: {
      CheckIcon,
      ChevronUpDownIcon,
      Listbox,
      ListboxButton,
      ListboxOption,
      ListboxOptions,
    },
    methods: {
      onProfileChange(selectedProfile) {
        this.$emit('profile-changed', selectedProfile);
      },
      loadAWSCredentials() {
        try {
          invoke('aws_credentials_index').then((body) => {
            try {
              // Check if body is a string that starts with "Error" or similar
              if (typeof body === 'string' && (body.startsWith('Error') || body.startsWith('error'))) {
                throw new Error(body);
              }
              this.profiles = JSON.parse(body);
            } catch (e) {
              console.log('Error parsing AWS credentials:', e);
              // Show mock profiles for browser testing or when API fails
              this.profiles = [
                { profile_name: 'default' },
                { profile_name: 'staging' },
                { profile_name: 'production' }
              ];
            }
          }).catch((error) => {
            console.error('Error loading AWS credentials:', error);
            // Show mock profiles for browser testing
            if (typeof window !== 'undefined' && !window.__TAURI__) {
              this.profiles = [
                { profile_name: 'default' },
                { profile_name: 'staging' },
                { profile_name: 'production' }
              ];
            }
          });
        } catch (error) {
          console.error('Tauri API not available:', error);
          // Show mock profiles for browser testing
          if (typeof window !== 'undefined' && !window.__TAURI__) {
            this.profiles = [
              { profile_name: 'default' },
              { profile_name: 'staging' },
              { profile_name: 'production' }
            ];
          }
        }
      },
    },
    mounted() {
      this.loadAWSCredentials();
      this.$emit('profile-changed', this.selectedProfile);
    },

  }
</script>
