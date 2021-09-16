<template>
  <div>
    Count from Server Side Event:
    <h3>{{ count }}</h3>
  </div>
</template>
<script>
export default {
  data() {
    return { count: 0 }
},
  mounted() {
    const source = new EventSource('http://localhost:3050/ticks');
    source.addEventListener('counter', (e) => {
      this.count = e.data;
      console.log(e.data)
    });
},
}
</script>
