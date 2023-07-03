<template>
  <div class="min-h-screen bg-gray-800 text-white p-5">
    <div class="flex justify-center">
      <img class="p-2" src="./assets/logo.png" height="50" width="50">
      <h1 class="p-1 flex-auto text-3xl font-bold font-mono">AWS Manager</h1>
      <ProfilesList @profile-changed="updateSelectedProfile"/>
    </div>
    <InstancesList :selectedProfile="selectedProfile"/>
  </div>
</template>

<script>
  import InstancesList from './components/InstancesList.vue';
  import ProfilesList from './components/ProfilesList.vue';

  export default {
    name: 'App',
    data() {
      return {
        selectedProfile: null,
      };
    },
    methods: {
      updateSelectedProfile(newProfile) {
        this.selectedProfile = newProfile;
      },
    },
    components: {
      InstancesList,
      ProfilesList
    },
  }
</script>
