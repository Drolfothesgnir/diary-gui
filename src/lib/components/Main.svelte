<script lang="ts">
  export let containerProps: svelteHTML.HTMLAttributes = {};
</script>

<main {...containerProps} class="main">
  <slot></slot>
</main>

<style lang="scss">
  @use "../scss/colors.scss" as colors;

  .main {
    padding: 1rem;
    flex: 1;
    background-color: colors.$secondary;
    display: flex;
    justify-content: center;
    overflow: auto;
    flex-wrap: wrap;
  }
</style>
