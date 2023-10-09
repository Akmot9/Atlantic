<template>
    <div>
      <h1>Interfaces disponibles</h1>
      <div class="button-container">
        <button class="button" v-for="netInterface in netInterfaces" :key="netInterface" @click="handleClick(netInterface)">
          {{ netInterface }}
        </button>
      </div>
    </div>
  </template>
  
  <script>
  import { invoke } from '@tauri-apps/api/tauri';
  
  export default {
    data() {
      return {
        netInterfaces: []
      };
    },
    methods: {
        async handleClick(netInterface) {
        console.log(`You clicked on interface: ${netInterface}`);
        await invoke('print_selected_interface', { interface_name: netInterface });
        // Here you can put any code to handle the button click.
      }
    },
    mounted() {
      invoke('get_interfaces').then((interfaces) => {
        this.netInterfaces = interfaces;
      });
    }
  };
  </script>
  
  <style scoped>
  .button-container {
    display: flex;
    flex-direction: column;
  }
  
  button {
  text-align: left;  /* Align text to the left */
  width: 100%;  /* Take up full width */
}
  </style>
  