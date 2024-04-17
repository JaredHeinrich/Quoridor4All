<script>
  import "../app.pcss";
</script>

<slot />

<style lang="postcss">
  :global(body) {
    @apply bg-gray-900;
  }
</style>
