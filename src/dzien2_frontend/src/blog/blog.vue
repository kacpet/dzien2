
<template>
    <h2 class="text-blue-600">Wpisy na bloga:</h2>
    <div v-for="(wpis, index) in wpisy">
        <h1>{{ index }}.</h1>
        <p>{{ wpis }}</p>
    </div>
    <div class="w-100">
        <button @click="pobierzWpisy" class="flex flex-row-reverse">tekst</button>
    </div>
<input v-model="nowyBlog" type="text" />
<button @click="dodajWpis">REFRESH</button>
</template>
<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';
export default {
    data(){
        return {
            wpisy: [],
            nowyBlog: ""
        }
    },
    methods: {
        async pobierzWpisy() {
            this.wpisy =  await dzien2_backend.odczytaj_wpisy();
        },
        async dodajWpis() {
            await dzien2_backend.dodaj_wpis(this.nowyBlog);
        },
        async mounted(){
        this.pobierzWpisy();
    }
    }
}
</script>