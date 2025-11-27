<script lang="ts">
  import { browser } from "$app/environment";

  let darkMode: boolean = $state(
    browser &&
      (localStorage.getItem("light-dark") ?? (window.matchMedia("(prefers-color-scheme: dark)").matches && "dark")) ===
        "dark"
  );
  let lightDarkClass = $derived(darkMode ? "dark" : "light");

  $effect(() => {
    localStorage.setItem("light-dark", lightDarkClass);
    document.documentElement.setAttribute("data-theme", lightDarkClass);
  });
</script>

<div class="light-dark-switcher">
  <label class="theme-toggle" title="Toggle theme">
    <input type="checkbox" bind:checked={darkMode} />
    <span class="theme-toggle-sr">Toggle theme</span>
    <svg
      xmlns="http://www.w3.org/2000/svg"
      aria-hidden="true"
      class="theme-toggle__within"
      height="1em"
      width="1em"
      viewBox="0 0 32 32"
      fill="currentColor"
    >
      <clipPath id="theme-toggle__within__clip">
        <path d="M0 0h32v32h-32ZM6 16A1 1 0 0026 16 1 1 0 006 16" />
      </clipPath>
      <g clip-path="url(#theme-toggle__within__clip)">
        <path
          d="M30.7 21.3 27.1 16l3.7-5.3c.4-.5.1-1.3-.6-1.4l-6.3-1.1-1.1-6.3c-.1-.6-.8-.9-1.4-.6L16 5l-5.4-3.7c-.5-.4-1.3-.1-1.4.6l-1 6.3-6.4 1.1c-.6.1-.9.9-.6 1.3L4.9 16l-3.7 5.3c-.4.5-.1 1.3.6 1.4l6.3 1.1 1.1 6.3c.1.6.8.9 1.4.6l5.3-3.7 5.3 3.7c.5.4 1.3.1 1.4-.6l1.1-6.3 6.3-1.1c.8-.1 1.1-.8.7-1.4zM16 25.1c-5.1 0-9.1-4.1-9.1-9.1 0-5.1 4.1-9.1 9.1-9.1s9.1 4.1 9.1 9.1c0 5.1-4 9.1-9.1 9.1z"
        />
      </g>
      <path
        class="theme-toggle__within__circle"
        d="M16 7.7c-4.6 0-8.2 3.7-8.2 8.2s3.6 8.4 8.2 8.4 8.2-3.7 8.2-8.2-3.6-8.4-8.2-8.4zm0 14.4c-3.4 0-6.1-2.9-6.1-6.2s2.7-6.1 6.1-6.1c3.4 0 6.1 2.9 6.1 6.2s-2.7 6.1-6.1 6.1z"
      />
      <path
        class="theme-toggle__within__inner"
        d="M16 9.5c-3.6 0-6.4 2.9-6.4 6.4s2.8 6.5 6.4 6.5 6.4-2.9 6.4-6.4-2.8-6.5-6.4-6.5z"
      />
    </svg>
  </label>
</div>

<style>
  .theme-toggle {
    font-size: 2rem;
    display: block;
    width: fit-content;
  }

  /*
    Modified version from theme-toggles package. Uses data-theme instead of :checked

    Full docs at toggles.dev
  */
  .theme-toggle {
    --theme-toggle__within--duration: 500ms;
  }
  .theme-toggle__within * {
    transform-origin: center;
    transition: transform calc(var(--theme-toggle__within--duration)) cubic-bezier(0, 0, 0, 1.25);
  }

  :root[data-theme="dark"] .theme-toggle input[type="checkbox"] ~ .theme-toggle__within {
    & .theme-toggle__within__circle {
      transform: scale(1.5);
    }
    & .theme-toggle__within__inner {
      transform: translate3d(3px, -3px, 0) scale(1.2);
    }
    & g path {
      transform: scale(0.65);
    }
  }

  .theme-toggle {
    border: none;
    background: none;
    cursor: pointer;
  }
  .theme-toggle input[type="checkbox"] {
    display: none;
  }
  .theme-toggle .theme-toggle-sr {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
  }

  @media (prefers-reduced-motion: reduce) {
    .theme-toggle:not(.theme-toggle--force-motion) * {
      transition: none !important;
    }
  }
</style>
