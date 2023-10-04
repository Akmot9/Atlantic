<script setup>
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const netInterfaces = ref([]);

onMounted(async () => {
  netInterfaces.value = await invoke('get_interfaces');
});
</script>


<template>
  <div>
    <h1>Available Interfaces</h1>
    <ul>
      <li v-for="netInterface in netInterfaces" :key="netInterface">{{ netInterface }}</li>
    </ul>
  </div>
</template>


<style scoped>
</style>
