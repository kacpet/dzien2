<template>
  <div>
      <h2 class="text-blue-600">Wpisy na bloga:</h2>
      <div class="w-100 flex flex-row-reverse">
          <button @click="pobierzWpisy" class=" rounded bg-blue-600 text-white p-4">refresh</button>
      </div>
      <div class="grid mx-6 gap-4 my-4">
      <div v-for="wpis in wpisy" class="bg-stone-300 drop-shadow-xl p-4" >
          <p>{{ wpis }}</p>
      </div>
      </div>
      <div class="grid flex-col justify-center">
        <input v-model="nowyBlog"class="border-2 border-blue-600 p-4 " type="text">
        <button @click="dodajWpisy" class="rounded bg-blue-600 text-white p-4">dodaj</button>
      </div>
  </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

export default{
  data(){
      return{
          wpisy:[],
          nowyBlog: ""
      }
  },
  methods:{
      async dodajWpisy(){
          this.wpisy = await dzien2_backend.dodaj_wpis(this.nowyBlog);
      },
      async pobierzWpisy(){
          this.wpisy = await dzien2_backend.odczytaj_wpisy();
      }
  },
  async mounted(){
      this.pobierzWpisy()
  }
}
</script>