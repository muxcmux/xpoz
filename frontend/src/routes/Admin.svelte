<script lang="ts">
  import { fly } from "svelte/transition";
</script>

<section class="page">
  <header in:fly="{{ x: -40, duration: 400 }}">
    <h1>
      <a href="/#/" title="Back to albums">
        <svg><use xlink:href="#i-chevron-left"/></svg>
      </a>
      <span>Admin</span>
    </h1>
  </header>
</section>

