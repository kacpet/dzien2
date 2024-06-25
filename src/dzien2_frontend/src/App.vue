<script setup>
import { ref } from 'vue';
import { dzien2_backend } from 'declarations/dzien2_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const LN = target.querySelector('#LN').value;
  const IQ = target.querySelector('#IQ').value;
  await dzien2_backend.greet(name,LN,Number(IQ)).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <label for="name">Enter your last name: &nbsp;</label>
      <input id="LN" alt="Name" type="text" /><br>
      <label for="name">Enter your IQ: &nbsp;</label>
      <input id="IQ" alt="Name" type="number" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>
