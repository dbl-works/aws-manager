<template>
  <Listbox v-model="selectedProfile" @update:model-value="onProfileChange">
    <div class="relative mt-1">
      <ListboxButton
        class="relative w-64 cursor-default rounded-lg bg-white py-2 pl-3 pr-10 text-left shadow-md focus:outline-none focus-visible:border-indigo-500 focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-orange-300 sm:text-sm"
      >
        <span class="block truncate text-black">{{ selectedProfile }}</span>
        <span
          class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2"
        >
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
          class="absolute mt-1 max-h-60 w-full min-w-max overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm"
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
                active ? 'bg-amber-100 text-amber-900' : 'text-gray-900',
                'relative cursor-default select-none py-2 pl-10 pr-4',
              ]"
            >
              <span
                :class="[
                  selected ? 'font-medium' : 'font-normal',
                  'block truncate',
                ]"
                >{{ profile.profile_name }}</span
              >
              <span
                v-if="selected"
                class="absolute inset-y-0 left-0 flex items-center pl-3 text-amber-600"
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
  import { invoke } from '@tauri-apps/api';
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
        invoke('aws_credentials_index').then((body) => {
          this.profiles = JSON.parse(body);
        })
      },
    },
    mounted() {
      this.loadAWSCredentials();
      this.$emit('profile-changed', this.selectedProfile);
    },

  }
</script>
